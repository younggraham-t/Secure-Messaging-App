use yew::prelude::*;
use crate::ui::components::sidebar::Sidebar;
use crate::ui::components::chat_window::ChatWindow;
use crate::ui::user_context::UserContextProvider;
use crate::ui::secure_mode_context::SecureModeContextProvider;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <UserContextProvider>
            <SecureModeContextProvider>
                <div class="app-container">
                    <Sidebar />
                    <ChatWindow />
                </div>
            </SecureModeContextProvider>
        </UserContextProvider>
    }
}
