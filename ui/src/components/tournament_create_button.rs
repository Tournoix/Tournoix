use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::routers::Route;

#[derive(PartialEq, Properties)]
pub struct TournamentCreateButtonProps {}

#[function_component]
pub fn TournamentCreateButton(props: &TournamentCreateButtonProps) -> Html {
    let TournamentCreateButtonProps {} = props;
    let navigator = use_navigator().unwrap();

    let on_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::TournoixCreate))
    };

    html! {
        <div class="tournament-card flex-col text-center" onclick={on_click}>
            <img src="/img/plus.svg" class="add-btn"/>
            {"Créer un tournoi"}
        </div>
    }
}
