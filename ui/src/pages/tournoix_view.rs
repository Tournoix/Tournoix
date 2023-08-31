use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{layouts::homelayout::HomeLayout, routers::Route, components::{backlink::Backlink, results::Results, qualificationPhase::QualificationPhase, eliminationPhase::EliminationPhase}};

#[derive(PartialEq, Properties)]
pub struct TournoixViewProps {
    pub id: i32,
}

#[function_component]
pub fn TournoixView(props: &TournoixViewProps) -> Html {
    let TournoixViewProps { id } = props;
    let navigator = use_navigator().unwrap();

    let on_click_edit = {
        let navigator = navigator.clone();
        let id = id.clone();
        Callback::from(move |_| navigator.push(&Route::TournoixEdit{ id }))
    };

    let on_click_match = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::BetView{ id: 42 }))
    };
    
    html! {
        <HomeLayout>
            <div class="flex flex-col items-center h-full pb-16 pt-12 sm:w-9/12 w-11/12 mx-auto relative">
                <Backlink route={Route::Tournoix} label="Retour à la liste des tournoix"/>
                <h1 class="mb-5">{"Affichage de tournoi"}</h1>
                <h2>{"Id du tournoi : "}{ id }</h2>
                <button class="bg-green-500 hover:bg-green-700 text-white font-bold p-2" onclick={on_click_edit}>{"MODIFIER CE TOURNOI (bouton affiché uniquement si on a les droits)"}</button>
                <button class="bg-green-500 hover:bg-green-700 text-white font-bold p-2" onclick={on_click_match}>{"AFFICHER UN MATCH DE TEST"}</button>
                <QualificationPhase/>
                <EliminationPhase/>
                <Results/>
            </div>
        </HomeLayout>
    }
}