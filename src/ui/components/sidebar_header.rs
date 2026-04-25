use yew::prelude::*;
use web_sys::HtmlSelectElement;
use crate::ui::user_context::UserContextValue;
use crate::ui::secure_mode_context::SecureModeContextValue;

#[function_component(SidebarHeader)]
pub fn sidebar_header() -> Html {
    let user_context = use_context::<UserContextValue>().expect("UserContext not found");
    let secure_context = use_context::<SecureModeContextValue>().expect("SecureModeContext not found");
    
    let users = vec!["Alice", "Bob", "Mallory"];

    let onchange_user = {
        let set_current_user = user_context.set_current_user.clone();
        Callback::from(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            set_current_user.emit(select.value());
        })
    };

    let ontoggle_secure = {
        let toggle = secure_context.toggle.clone();
        Callback::from(move |_| toggle.emit(()))
    };

    html! {
        <div class="sidebar-header">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                <h1 style="margin: 0;">{ "Chats" }</h1>
                
                <div style="display: flex; align-items: center; gap: 8px;">
                    <div style={format!("padding: 4px 8px; border-radius: 12px; font-size: 11px; font-weight: bold; color: white; background: {};", if secure_context.is_secure { "#00a400" } else { "#fa3e3e" })}>
                        { if secure_context.is_secure { "SECURE" } else { "INSECURE" } }
                    </div>
                    
                    <label class="switch">
                        <input type="checkbox" checked={secure_context.is_secure} onchange={ontoggle_secure} />
                        <span class="slider"></span>
                    </label>
                </div>
            </div>

            <div style="font-size: 11px; color: #65676b; margin-bottom: 8px; font-family: monospace;">
                { format!("Endpoint: {}", secure_context.endpoint) }
            </div>

            <div style="font-size: 13px; color: #65676b; margin-bottom: 4px;">{ "Signed in as:" }</div>
            <select onchange={onchange_user} style="width: 100%; padding: 8px; border-radius: 8px; border: 1px solid #ddd; background: #f0f2f5; outline: none; cursor: pointer;">
                    <option value="" selected={user_context.state.current_user.is_empty()} >
                    {"Select A User"}
                    </option>
                { for users.iter().map(|user| html! {
                    <option value={*user} selected={*user == user_context.state.current_user}>
                        { user }
                    </option>
                }) }
            </select>
        </div>
    }
}
