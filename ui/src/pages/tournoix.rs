use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::routers::Route;

#[derive(PartialEq, Properties)]
pub struct TournoixProps {}

#[function_component]
pub fn Tournoix(props: &TournoixProps) -> Html {
    let TournoixProps {} = props;
    let navigator = use_navigator().unwrap();
    
    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div class={"flex flex-col"}>
            {"Tournoixxx"}
            <button {onclick}>{"Go to Home"}</button>
        </div>
    }
}