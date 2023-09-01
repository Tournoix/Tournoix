use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{layouts::homelayout::HomeLayout, routers::Route, components::{backlink::Backlink, results::Results, qualificationPhase::QualificationPhase, bracket::Bracket, join_code::JoinCode, groups::{Groups, Group}}, utils::utils::fetch_tournament};

#[derive(PartialEq, Properties)]
pub struct TournoixViewProps {
    pub id: i32,
}

#[function_component]
pub fn TournoixView(props: &TournoixViewProps) -> Html {
    let TournoixViewProps { id } = props;
    let navigator = use_navigator().unwrap();
    
    let tournament = use_state(|| fetch_tournament(*id));

    // TODO Wheter or not the current user can edit this tournament
    let can_edit_tournament = true;

    let groups: UseStateHandle<Vec<Group>> = use_state(|| vec![
        Group { id: 0, name: "test0".to_string() },
        Group { id: 1, name: "test1".to_string() },
        Group { id: 2, name: "test2".to_string() },
        Group { id: 3, name: "test3".to_string() },
        Group { id: 4, name: "test4".to_string() },
        Group { id: 5, name: "test5".to_string() },
        Group { id: 6, name: "test6".to_string() },
        Group { id: 7, name: "test7".to_string() },
    ]);

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
                <button class="m-3 bg-green-500 hover:bg-green-700 text-white font-bold p-2" onclick={on_click_match}>{"AFFICHER UN MATCH DE TEST"}</button>
                <h1 class="mb-5">{tournament.name.to_string()}</h1>
                {if can_edit_tournament { html! {<a onclick={on_click_edit} class="a_link mb-6">{"Modifier ce tournoi"}</a>}} else { html! {} }}
                <JoinCode code={tournament.code.to_string()}/>
                <hr/>
                <h2>{"Informations"}</h2>
                <div>{"Date: "}{tournament.date.to_string()}</div>
                <div>{"Lieu: "}{tournament.location.to_string()}</div>
                <div>{"Description: "}{tournament.description.to_string()}</div>
                <hr/>
                <h2>{"Phase de qualifications"}</h2>
                <ContextProvider<UseStateHandle<Vec<Group>>> context={groups.clone()}>
                    <Groups/>
                </ContextProvider<UseStateHandle<Vec<Group>>>>
                <QualificationPhase/>
                <hr/>
                <h2>{"Phase d'éliminations"}</h2>
                /*<Bracket/>*/
                <hr/>
                <h2>{"Résultats"}</h2>
                <div class="text-red-500">{"AFFICHER UNIQUEMENT SI TOUT LES MATCHS DE CE TOURNOIS SONT TERMINÉS"}</div>
                <Results/>
            </div>
        </HomeLayout>
    }
}