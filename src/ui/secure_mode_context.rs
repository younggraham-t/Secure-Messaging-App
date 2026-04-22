use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SecureModeContextValue {
    pub is_secure: bool,
    pub endpoint: String,
    pub toggle: Callback<()>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(SecureModeContextProvider)]
pub fn secure_mode_context_provider(props: &Props) -> Html {
    let is_secure = use_state(|| true);

    let toggle = {
        let is_secure = is_secure.clone();
        Callback::from(move |_| {
            is_secure.set(!*is_secure);
        })
    };

    let endpoint = if *is_secure {
        "/api/ca/public-key".to_string()
    } else {
        "/api/user/public-key".to_string()
    };

    let context = SecureModeContextValue {
        is_secure: *is_secure,
        endpoint,
        toggle,
    };

    html! {
        <ContextProvider<SecureModeContextValue> context={context}>
            { props.children.clone() }
        </ContextProvider<SecureModeContextValue>>
    }
}
