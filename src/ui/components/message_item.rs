use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MessageProps {
    pub text: AttrValue,
    pub is_sent: bool,
}

#[function_component(MessageItem)]
pub fn message_item(props: &MessageProps) -> Html {
    let class = if props.is_sent { "message sent" } else { "message received" };
    
    html! {
        <div {class}>
            { &props.text }
        </div>
    }
}
