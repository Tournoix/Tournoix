use std::{fmt, str::FromStr};

use yew::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NotifType {
    Success,
    Warning,
    Error
}
impl fmt::Display for NotifType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NotifType::Success => write!(f, "success"),
            NotifType::Warning => write!(f, "warning"),
            NotifType::Error => write!(f, "error"),
        }
    }
}
impl FromStr for NotifType {
    type Err = ();

    fn from_str(input: &str) -> Result<NotifType, Self::Err> {
        match input {
            "success"  => Ok(NotifType::Success),
            "warning"  => Ok(NotifType::Warning),
            "error"  => Ok(NotifType::Error),
            _      => Err(()),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Notif {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub type_notif: NotifType,
}

#[derive(PartialEq, Properties)]
pub struct NotificationProps {
    pub notif: Notif
}

#[function_component]
pub fn Notification(props: &NotificationProps) -> Html {
    let NotificationProps { notif } = props;
    let notifs = use_context::<UseStateHandle<Vec<Notif>>>().expect("Missing notifs provider");

    let on_click = {
            let notifs = notifs.clone();
            let notif = notif.clone();

            Callback::from(
            move |_| {
                let mut notifs_copy = (*notifs).clone();

                let target_id = notif.id;
                notifs_copy.retain(|notif| notif.id != target_id);

                notifs.set(notifs_copy);
                ()
            }
        )
    };

    html! {
        <div onclick={on_click} class={format!("m-2 p-3 text-white rounded drop-shadow-lg cursor-pointer pointer-events-auto notif-{}", {&notif.type_notif})}>
            <h3>{&notif.title}</h3>
            {&notif.content}
        </div>
    }
}