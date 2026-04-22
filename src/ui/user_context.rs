use yew::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct UserState {
    pub current_user: String,
    pub active_recipient: String,
    /// Maps username to the 32-byte AES session key
    pub session_keys: HashMap<String, Vec<u8>>,
}

impl UserState {
    pub fn get_active_session_key(&self) -> Option<&Vec<u8>>{
        return self.session_keys.get(&self.active_recipient);
    }
}
#[derive(Clone, PartialEq)]
pub struct UserContextValue {
    pub state: UserState,
    pub set_current_user: Callback<String>,
    pub set_recipient: Callback<String>,
    pub add_session_key: Callback<(String, Vec<u8>)>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}


#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &Props) -> Html {
    let current_user = use_state(|| "Alice".to_string());
    let active_recipient = use_state(|| "Bob".to_string());
    let session_keys = use_state(|| HashMap::<String, Vec<u8>>::new());

    let set_current_user = {
        let current_user = current_user.clone();
        Callback::from(move |new_user: String| {
            current_user.set(new_user);
        })
    };

    let set_recipient = {
        let active_recipient = active_recipient.clone();
        Callback::from(move |new_recipient: String| {
            active_recipient.set(new_recipient);
        })
    };

    let add_session_key = {
        let session_keys = session_keys.clone();
        Callback::from(move |(user, key): (String, Vec<u8>)| {
            let mut keys = (*session_keys).clone();
            keys.insert(user, key);
            session_keys.set(keys);
        })
    };

    let context = UserContextValue {
        state: UserState {
            current_user: (*current_user).clone(),
            active_recipient: (*active_recipient).clone(),
            session_keys: (*session_keys).clone(),
        },
        set_current_user,
        set_recipient,
        add_session_key,
    };

    html! {
        <ContextProvider<UserContextValue> context={context}>
            { props.children.clone() }
        </ContextProvider<UserContextValue>>
    }
}
