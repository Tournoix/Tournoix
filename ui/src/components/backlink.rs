use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[derive(PartialEq, Properties)]
pub struct BacklinkProps {
    pub label: String,
    pub route: crate::routers::Route,
}

#[function_component]
pub fn Backlink(props: &BacklinkProps) -> Html {
    let BacklinkProps { route, label } = props;
    let navigator = use_navigator().unwrap();
    let route = route.clone();

    let on_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&route))
    };

    html! {
        <a onclick={on_click} class="backlink">{"<< "}{label}</a>
    }
}