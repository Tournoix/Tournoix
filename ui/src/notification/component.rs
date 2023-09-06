use yew::{classes, function_component, html, Callback, Html, MouseEvent, Properties};

use super::CustomNotification;

fn format_date_time(datetime: &time::OffsetDateTime) -> String {
    format!(
        "{}:{}:{} {}.{}.{}",
        datetime.hour(),
        datetime.minute(),
        datetime.second(),
        datetime.day(),
        datetime.month(),
        datetime.year()
    )
}

#[derive(Properties, Clone, PartialEq)]
pub struct CustomNotificationComponentProps {
    pub notification: CustomNotification,
    pub onclick: Callback<MouseEvent>,
    pub onenter: Callback<MouseEvent>,
    pub onleave: Callback<MouseEvent>,
}

#[function_component(CustomNotificationComponent)]
pub fn custom_notification_component(props: &CustomNotificationComponentProps) -> Html {
    let title = &props.notification.title;
    let content = &props.notification.content;
    let type_notif = &props.notification.type_notif;
    let spawn_time = &props.notification.spawn_time;

    let onclick = props.onclick.clone();
    let onenter = props.onenter.clone();
    let onleave = props.onleave.clone();

    html! {
        <div onclick={onclick} onmouseenter={onenter} onmouseleave={onleave} class={format!("m-2 p-3 text-white rounded drop-shadow-lg cursor-pointer pointer-events-auto notif-{}", {(type_notif).to_string().to_lowercase().as_str()})}>
            <h3>{title}</h3>
            <div>
                {content}
            </div>
            <div class={classes!("time", "mt-1")}>{format_date_time(spawn_time)}</div>
        </div>
    }
}
