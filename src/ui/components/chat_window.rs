use crate::api::fetch::fetch_public_key;
use crate::crypto::session::{encrypt_message, initiate_session};
use crate::ui::components::message_item::MessageItem;
use crate::ui::secure_mode_context::SecureModeContextValue;
use crate::ui::user_context::{UserContextValue, ChatMessage};
use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(ChatWindow)]
pub fn chat_window() -> Html {
    let user_context = use_context::<UserContextValue>().expect("UserContext not found");
    let message = use_state(|| "".to_string());
    let secure_mode_context =
        use_context::<SecureModeContextValue>().expect("SecureModeContext not found");

    let oninput = {
        let message = message.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            message.set(input.value());
        })
    };

    let onsubmit = {
        let message = message.clone();
        let user_context = user_context.clone();
        let secure_mode_context = secure_mode_context.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if !message.is_empty() {
                let message_text = (*message).clone();
                let user_context = user_context.clone();
                let secure_mode_context = secure_mode_context.clone();

                spawn_local(async move {
                    let recipient = user_context.state.active_recipient.to_lowercase();
                    let endpoint = secure_mode_context.endpoint;

                    // 1. Get or Create Session Key
                    let session_key = match user_context.state.session_keys.get(&recipient) {
                        Some(key) => key.clone(),
                        None => {
                            let pub_key = fetch_public_key(&recipient, &endpoint).await.expect("Failed to fetch public key");
                            let session_init = initiate_session(&pub_key);
                            
                            user_context.add_session_key.emit((recipient.clone(), session_init.session_key.clone()));
                            user_context.send_message.emit(json!({
                                "type": "session_start", 
                                "to": recipient.clone(), 
                                "encrypted_key": session_init.encrypted_key
                            }));
                            
                            session_init.session_key
                        }
                    };

                    // 2. Encrypt and Send Message
                    let payload = encrypt_message(&message_text, session_key);
                    user_context.send_message.emit(json!({
                        "type": "message", 
                        "to": recipient.clone(), 
                        "payload": payload
                    }));

                    // 3. Add to local UI history
                    user_context.add_message.emit(ChatMessage {
                        from: "Me".to_string(),
                        text: message_text,
                    });
                });

                message.set("".to_string());
            }
        })
    };

    html! {
        <div class="main-chat">
            <div class="chat-header">
                <div class="avatar" style="width: 32px; height: 32px; margin-right: 10px; background: #0084ff;"></div>
                <div style="display: flex; flex-direction: column;">
                    <div style="font-weight: 600;">{ &user_context.state.active_recipient }</div>
                    <div style="font-size: 11px; color: #65676b;">{ format!("Chatting as {}", user_context.state.current_user) }</div>
                </div>
            </div>

            <div class="messages-area">
                { for user_context.state.messages.iter().map(|msg| {
                    let is_sent = msg.from == "Me";
                    html! {
                        <MessageItem text={msg.text.clone()} {is_sent} />
                    }
                }) }
            </div>

            <form class="input-area" {onsubmit}>
                <input type="text" placeholder="Aa" value={(*message).clone()} {oninput}/>
                <button class="send-btn" type="submit" style="background: none; border: none;">{ "Send" }</button>
            </form>
        </div>
    }
}
