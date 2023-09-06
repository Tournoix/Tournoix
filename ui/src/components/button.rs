use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or_default]
    pub class: Classes,
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let ButtonProps {
        class,
        children,
        onclick,
        disabled,
    } = props;

    html! {
        <button class={classes!(class.clone(), "bg-nut", "text-white", "rounded", "drop-shadow-md", "hover:duration-[200ms]", "duration-[600ms]", "transition-all", if *disabled {"bg-nutLight cursor-not-allowed"} else {"hover:tracking-[.05em]"})} onclick={onclick} disabled={*disabled}>{children.clone()}</button>
    }
}
