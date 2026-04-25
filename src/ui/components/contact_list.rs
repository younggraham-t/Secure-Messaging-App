use yew::prelude::*;
use crate::ui::user_context::UserContextValue;

#[function_component(ContactList)]
pub fn contact_list() -> Html {
    let user_context = use_context::<UserContextValue>().expect("UserContext not found");
    let users = vec!["Alice", "Bob", "Mallory"];

    html! {
        <>
            <div style="padding: 16px 16px 8px; font-size: 12px; font-weight: bold; color: #65676b; text-transform: uppercase; letter-spacing: 0.5px;">
                { "All Contacts" }
            </div>
            { for users.into_iter().filter(|u| u != &user_context.state.current_user).map(|user| {
                let onclick = {
                    let set_recipient = user_context.set_recipient.clone();
                    let user = user.to_string();
                    Callback::from(move |_| set_recipient.emit(user.clone()))
                };
                
                let is_active = user == user_context.state.active_recipient;

                html! {
                    <div {onclick} class="chat-item" style={if is_active { "background-color: #e7f3ff;" } else { "" }}>
                        <div class="avatar" style={if is_active { "background: #0084ff;" } else { "" }}></div>
                        <div>
                            <div style="font-weight: 600;">{ user }</div>
                            <div style="font-size: 13px; color: #65676b;">{ "Available" }</div>
                        </div>
                    </div>
                }
            }) }
        </>
    }
}
