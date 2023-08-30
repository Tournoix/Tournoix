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

#[derive(Clone)]
pub struct Notif {
    pub title: String,
    pub content: String,
    pub type_notif: NotifType,
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