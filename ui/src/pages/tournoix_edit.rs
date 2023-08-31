use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{layouts::homelayout::HomeLayout, components::{backlink::Backlink, results::Results, eliminationPhase::EliminationPhase, qualificationPhase::QualificationPhase}, routers::Route};

#[derive(PartialEq, Properties)]
pub struct TournoixEditProps {
    pub id: i32,
}

#[function_component]
pub fn TournoixEdit(props: &TournoixEditProps) -> Html {
    let TournoixEditProps { id } = props;
    let navigator = use_navigator().unwrap();

    let on_click_view = {
        let navigator = navigator.clone();
        let id = id.clone();
        Callback::from(move |_| navigator.push(&Route::TournoixView{ id }))
    };
    
    html! {
        <HomeLayout>
            <div class="flex flex-col items-center h-full pb-16 pt-12 sm:w-9/12 w-11/12 mx-auto relative">
                <Backlink route={Route::Tournoix} label="Retour à la liste des tournoix"/>
                <h1 class="mb-5">{"Modification de tournoi"}</h1>
                <h2>{"Id du tournoi : "}{ id }</h2>
                <button class="bg-green-500 hover:bg-green-700 text-white font-bold p-2" onclick={on_click_view}>{"VOIR CE TOURNOI COMME UN UTILISATEUR"}</button>
                <QualificationPhase/>
                <EliminationPhase/>
                <Results/>
            </div>
        </HomeLayout>
    }
}