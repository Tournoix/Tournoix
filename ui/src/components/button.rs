use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    pub label: String,
    pub onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let ButtonProps { label, onclick } = props;

    html! {
        <button class="bg-white px-5 py-3 rounded border drop-shadow-md hover:scale-125 hover:duration-[200ms] duration-[600ms] transition-all" onclick={onclick}>{label}</button>
    }
}