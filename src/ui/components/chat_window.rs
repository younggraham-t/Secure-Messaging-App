use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use wasm_bindgen_futures::spawn_local;
use yew::{prelude::*};
use crate::api::fetch::fetch_public_key;
use crate::crypto::session::initiate_session;
use crate::ui::components::message_item::MessageItem;
use crate::ui::secure_mode_context::{SecureModeContextValue};
use crate::ui::user_context::{UserContextValue};

#[function_component(ChatWindow)]
pub fn chat_window() -> Html {
    let user_context = use_context::<UserContextValue>().expect("UserContext not found");
    let message = use_state(|| "".to_string());
    let secure_mode_context = use_context::<SecureModeContextValue>().expect("SecureModeContext not found");

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
                log::info!("[{}] Sending message to {}: {}", user_context.state.current_user, user_context.state.active_recipient, *message);
                
                let message = message.clone();
                let user_context = user_context.clone();
                let secure_mode_context = secure_mode_context.clone();
                
                spawn_local(async move {
                    
                    let user_context = user_context.clone();
                    let secure_mode_context = secure_mode_context.clone();
                    
                    let endpoint = secure_mode_context.endpoint;
                    let recipient = user_context.state.active_recipient.to_lowercase();
                    let res = fetch_public_key(&recipient, &endpoint).await;
                    
                    let pub_key = match res {
                        Ok(data) => {
                            log::info!("got public key: {}", &data);
                            data
                        },
                        Err(e) => {
                            log::error!("Error from public key fetch: {}", e);
                            panic!()
                        }

                    };
                    

                    let session_key = initiate_session(&pub_key);
                    user_context.add_session_key.emit((recipient, session_key.session_key));
                    
                    

                                        
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
                <MessageItem text={format!("Hey! This is a secure channel with {}.", user_context.state.active_recipient)} is_sent={false} />
            </div>

            <form class="input-area" {onsubmit}>
                <input type="text" placeholder="Aa" value={(*message).clone()} {oninput}/>
                <button class="send-btn" type="submit" style="background: none; border: none;">{ "Send" }</button>
            </form>
        </div>
    }
}
