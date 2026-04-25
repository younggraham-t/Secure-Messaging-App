use serde_json::json;
use std::{cell::Cell, collections::HashMap, rc::Rc};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
use futures::{SinkExt, StreamExt, channel::mpsc::unbounded};

use crate::crypto::session::{MessagePayload, decrypt_message, receive_session};

#[derive(Clone, Debug, PartialEq)]
pub struct ChatMessage {
    pub from: String,
    pub text: String,
}

enum MessageAction {
    Add(ChatMessage)
}

#[derive(Clone, Debug, PartialEq, Default)]
struct MessageState {
    pub messages: Vec<ChatMessage>,
}
impl Reducible for MessageState {
    type Action = MessageAction;
    
    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        
        let mut messages = self.messages.clone();
        match action {
            MessageAction::Add(message) => messages.push(message)
        };
        
        Self { messages }.into()
        
    }

}

#[derive(Clone, Debug, PartialEq)]
pub struct UserState {
    pub current_user: String,
    pub active_recipient: String,
    pub session_keys: HashMap<String, Vec<u8>>,
    pub private_key: Option<String>,
    pub messages: Vec<ChatMessage>,
}

#[derive(Clone, PartialEq)]
pub struct UserContextValue {
    pub state: UserState,
    pub set_current_user: Callback<String>,
    pub set_recipient: Callback<String>,
    pub add_session_key: Callback<(String, Vec<u8>)>,
    pub add_message: Callback<ChatMessage>,
    pub send_message: Callback<serde_json::Value>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

fn get_private_key(user: &str) -> Option<String> {
    match user {
        "Alice" => Some(include_str!("../../keys/alice_private.pem").to_string()),
        "Bob" => Some(include_str!("../../keys/bob_private.pem").to_string()),
        "Mallory" => Some(include_str!("../../keys/mallory_private.pem").to_string()),
        _ => None,
    }
}

#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &Props) -> Html {
    let current_user = use_state(|| "".to_string());
    let active_recipient = use_state(|| "".to_string());
    let session_keys = use_state(HashMap::<String, Vec<u8>>::new);
    let private_key = use_state(|| get_private_key("Alice"));
    let messages = use_reducer(MessageState::default);
    
    let tx_state = use_state(|| unbounded::<Message>().0);

    let add_message_cb = {
        let messages = messages.clone();
        Callback::from(move |msg: ChatMessage| {
            messages.dispatch(MessageAction::Add(msg));
        })
    };

    let set_current_user_cb = {
        let current_user = current_user.clone();
        Callback::from(move |new_user: String| {
            current_user.set(new_user);
        })
    };

    let set_recipient_cb = {
        let active_recipient = active_recipient.clone();
        Callback::from(move |new_recipient: String| {
            active_recipient.set(new_recipient);
        })
    };

    let add_session_key_cb = {
        let session_keys = session_keys.clone();
        Callback::from(move |(user, key): (String, Vec<u8>)| {
            let mut keys = (*session_keys).clone();
            keys.insert(user, key);
            session_keys.set(keys);
        })
    };

    let send_message_cb = {
        let tx_state = tx_state.clone();
        Callback::from(move |json: serde_json::Value| {
            let _ = tx_state.unbounded_send(Message::Text(json.to_string()));
        })
    };

    // EFFECT: Manage WebSocket Lifecycle
    {
        // clone needed dependencies for use effect
        let current_user = (*current_user).clone();
        let tx_state = tx_state.clone();
        let private_key = private_key.clone();
        let add_session_key = add_session_key_cb.clone();
        let add_message = add_message_cb.clone();

        use_effect_with(current_user, move |user| {
            let is_active = Rc::new(Cell::new(true));
            let is_active_for_task = is_active.clone();
            if !user.is_empty() {
                let user = user.clone();

                // Update the local private key state
                private_key.set(get_private_key(&user));

                // for future uses of this key in this specific effect call
                let resolved_p_key = get_private_key(&user);
                private_key.set(resolved_p_key.clone());
                
                let (new_tx, mut new_rx) = unbounded::<Message>();
                tx_state.set(new_tx);
                
                let add_message = add_message.clone();
                let add_session_key = add_session_key.clone();
                
                spawn_local(async move {
                    log::info!("[WS] Connecting to Server for {}...", user);
                    
                    let ws = match WebSocket::open("ws://localhost:3000") {
                        Ok(ws) => ws,
                        Err(e) => {
                            log::error!("[WS] Connection failed: {:?}", e);
                            return;
                        }
                    };

                    let (mut write, mut read) = ws.split();
                    let reg = json!({ "type": "register", "username": user });
                    write.send(Message::Text(reg.to_string())).await.ok();

                    spawn_local(async move {
                        while let Some(msg) = new_rx.next().await {
                            let _ = write.send(msg).await;
                        }
                    });

                    // keep track of the sessions independent of UX logic
                    let mut local_sessions: HashMap<String, Vec<u8>> = HashMap::new();
                    // recieve a message (bob)
                    while let Some(msg) = read.next().await {
                        if !is_active_for_task.get() {
                            break;
                        }
                        if let Ok(Message::Text(text)) = msg {
                            if let Ok(data) = serde_json::from_str::<serde_json::Value>(&text) {
                                match data["type"].as_str() {
                                    Some("session_receive") => {
                                        let sender = data["from"].as_str().expect("Failed to get from field").to_string();
                                        let encrypted_key = data["encrypted_key"].as_str().expect("Failed to get encrypted_key");
                                        if let Some(pk) = &resolved_p_key {
                                            log::info!("[WS] Incoming handshake from {}: {}", data["from"], &data["encrypted_key"]);
                                            let decrypted_key = receive_session(encrypted_key, pk);
                                            add_session_key.emit((sender.clone(), decrypted_key.clone()));
                                            local_sessions.insert(sender, decrypted_key);
                                        }
                                    },
                                    Some("message_receive") => {
                                        let payload: MessagePayload = serde_json::from_value::<MessagePayload>(data["payload"].clone()).expect("Failed to get message payload");
                                        let sender = data["from"].as_str().expect("Failed to get from field").to_string();
                                        log::info!("[WS] Incoming message from {}: {:#?}", &sender,  &payload);
                                        log::info!("[WS] Decrypting message");
                                        if let Some(session_key) = local_sessions.get(&sender) {
                                            if let Ok(message) = decrypt_message(&payload, session_key) {
                                                log::info!("[WS] Message decrypted: {}", message);
                                                add_message.emit(ChatMessage { from: sender, text: message });
                                            }
                                        }
                                    },
                                    _ => {}
                                }
                            }
                        }
                    }
                });
                
            }
            // move || {
            //     is_active.set(false);
            // };
        });
    }
    
    

    let context = UserContextValue {
        state: UserState {
            current_user: (*current_user).clone(),
            active_recipient: (*active_recipient).clone(),
            session_keys: (*session_keys).clone(),
            private_key: (*private_key).clone(),
            messages: (*messages).messages.clone(),
        },
        set_current_user: set_current_user_cb,
        set_recipient: set_recipient_cb,
        add_session_key: add_session_key_cb,
        add_message: add_message_cb,
        send_message: send_message_cb,
    };

    html! {
        <ContextProvider<UserContextValue> context={context}>
            { props.children.clone() }
        </ContextProvider<UserContextValue>>
    }
}
