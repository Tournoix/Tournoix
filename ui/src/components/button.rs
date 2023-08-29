use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or_default]
    pub class: Classes,
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let ButtonProps { class, children, onclick } = props;

    html! {
        <button class={classes!(class.clone(), "bg-nut", "text-white", "hover:tracking-[.05em]", "rounded", "drop-shadow-md", "hover:duration-[200ms]", "duration-[600ms]", "transition-all")} onclick={onclick}>{children.clone()}</button>
    }
}