use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{layouts::homelayout::HomeLayout, components::{backlink::Backlink, eliminationPhase::EliminationPhase, qualificationPhase::QualificationPhase, groups::{Groups, Group}}, routers::Route};

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
    
    let on_create_group_click = Callback::from(|_| ());

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
    
    html! {
        <HomeLayout>
            <div class="flex flex-col items-center h-full pb-16 pt-12 sm:w-9/12 w-11/12 mx-auto relative">
                <Backlink route={Route::Tournoix} label="Retour à la liste des tournoix"/>
                <h1 class="mb-5">{"Modification de tournoi"}</h1>
                <h2>{"Id du tournoi : "}{ id }</h2>
                <button class="m-3 bg-green-500 hover:bg-green-700 text-white font-bold p-2" onclick={on_click_view}>{"VOIR CE TOURNOI COMME UN UTILISATEUR"}</button>
                <h2>{"Général"}</h2>
                <hr/>
                <h2>{"Phase de qualifications"}</h2>
                <ContextProvider<UseStateHandle<Vec<Group>>> context={groups.clone()}>
                    <Groups on_create={on_create_group_click}/>
                </ContextProvider<UseStateHandle<Vec<Group>>>>
                <QualificationPhase/>
                <hr/>
                <h2>{"Phase d'éliminations"}</h2>
                <EliminationPhase/>
            </div>
        </HomeLayout>
    }
}