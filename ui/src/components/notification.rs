use yew::prelude::*;

#[derive(Clone)]
pub struct Notif {
    pub title: String,
    pub content: String,
}

#[derive(PartialEq, Properties)]
pub struct NotificationProps {}

#[function_component]
pub fn Notification(props: &NotificationProps) -> Html {
    let NotificationProps {} = props;

    html! {
        <div>{"Notif example"}</div>
    }
}