use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or_default]
    pub class: Classes,
    pub label: String,
    pub onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let ButtonProps { class, label, onclick } = props;

    html! {
        <button class={classes!("bg-white", "px-5", "py-3", "rounded", "border", "drop-shadow-md", "hover:scale-125", "hover:duration-[200ms]", "duration-[600ms]", "transition-all", class.clone())} onclick={onclick}>{label}</button>
    }
}