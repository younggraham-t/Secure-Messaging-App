use yew::prelude::*;
use crate::ui::user_context::UserContextValue;

#[function_component(SecureSessionList)]
pub fn secure_session_list() -> Html {
    let user_context = use_context::<UserContextValue>().expect("UserContext not found");

    if user_context.state.session_keys.is_empty() {
        return html! {};
    }

    html! {
        <>
            <div style="padding: 16px 16px 8px; font-size: 12px; font-weight: bold; color: #65676b; text-transform: uppercase; letter-spacing: 0.5px;">
                { "Secure Sessions" }
            </div>
            { for user_context.state.session_keys.keys().map(|user| {
                let is_active = user == &user_context.state.active_recipient;
                let user_clone = user.clone();
                let onclick = {
                    let set_recipient = user_context.set_recipient.clone();
                    Callback::from(move |_| set_recipient.emit(user_clone.clone()))
                };

                html! {
                    <div {onclick} class="chat-item" style={if is_active { "background-color: #e7f3ff;" } else { "" }}>
                        <div class="avatar" style="background: #00a400; display: flex; align-items: center; justify-content: center; color: white; font-size: 20px;">
                            { "✓" }
                        </div>
                        <div>
                            <div style="font-weight: 600;">{ user }</div>
                            <div style="font-size: 12px; color: #00a400;">{ "Handshake Complete" }</div>
                        </div>
                    </div>
                }
            }) }
            <div style="margin: 8px 16px; border-bottom: 1px solid #eee;"></div>
        </>
    }
}
