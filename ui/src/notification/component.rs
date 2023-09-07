use yew::{
    classes, function_component, html, use_effect_with_deps, use_state, Callback, Html, MouseEvent,
    Properties,
};
use yew_hooks::use_timeout;

use super::CustomNotification;

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
    let lifetime = &props.notification.lifetime;

    let in_state = use_state(|| true);
    let out_state = use_state(|| false);

    let onclick = props.onclick.clone();
    let onenter = props.onenter.clone();
    let onleave = props.onleave.clone();

    {
        let in_state = in_state.clone();

        use_timeout(
            move || {
                in_state.set(false);
            },
            1,
        );
    }

    {
        let lifetime = lifetime.clone();
        let out_state = out_state.clone();

        use_effect_with_deps(
            move |lifetime| {
                if lifetime.as_seconds_f32() <= 0.0 {
                    out_state.set(true);
                } else {
                    out_state.set(false);
                }
            },
            lifetime,
        );
    }

    html! {
        <div onclick={onclick} onmouseenter={onenter} onmouseleave={onleave} class={classes!("z-50", "m-2", "p-3", "text-white", "rounded", "drop-shadow-lg", "cursor-pointer", "pointer-events-auto", "transition-all", "duration-250", if *in_state {"translate-y-32 opacity-0"} else {if !*out_state {"opacity-100"} else {"opacity-0"}}, format!("notif-{}", (type_notif).to_string().to_lowercase().as_str()))}>
            <h3>{title}</h3>
            <div>
                {content}
            </div>
        </div>
    }
}
