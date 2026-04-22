use yew::prelude::*;
use crate::ui::components::sidebar_header::SidebarHeader;
use crate::ui::components::secure_session_list::SecureSessionList;
use crate::ui::components::contact_list::ContactList;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    html! {
        <div class="sidebar">
            <SidebarHeader />
            <div class="chat-list">
                <SecureSessionList />
                <ContactList />
            </div>
        </div>
    }
}
