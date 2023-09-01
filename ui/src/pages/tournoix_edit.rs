use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{layouts::homelayout::HomeLayout, components::{backlink::Backlink, bracket::Bracket, qualificationPhase::QualificationPhase, groups::{Groups, Group}, button::Button, teams::{Teams, Team}, form_input::FormInput, join_code::JoinCode}, routers::Route};

#[derive(PartialEq, Properties)]
pub struct TournoixEditProps {
    pub id: i32,
}

#[function_component]
pub fn TournoixEdit(props: &TournoixEditProps) -> Html {
    let TournoixEditProps { id } = props;
    let navigator = use_navigator().unwrap();

    let super_secret = "mCCx34d";

    let teams: UseStateHandle<Vec<Team>> = use_state(|| vec![
        Team { id: 0, is_being_edited: false, name: "Cloud9".to_string() },
        Team { id: 1, is_being_edited: false, name: "FaZe Clan".to_string() },
        Team { id: 2, is_being_edited: false, name: "NaVi".to_string() },
        Team { id: 3, is_being_edited: false, name: "NRG Esports".to_string() },
        Team { id: 4, is_being_edited: false, name: "G2 Esports".to_string() },
        Team { id: 5, is_being_edited: false, name: "fnatic".to_string() },
        Team { id: 6, is_being_edited: false, name: "Team with a comically long name".to_string() },
        Team { id: 7, is_being_edited: false, name: "Team 42".to_string() }
    ]);

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
                <a onclick={on_click_view} class="a_link mb-6">{"Voir ce tournoi en mode affichage"}</a>
                <JoinCode code={super_secret}/>
                <hr/>
                <h2>{"Général"}</h2>
                    <div class="flex flex-row w-full justify-center gap-5 lg:flex-nowrap flex-wrap">
                        <div class="w-1/2">
                            <FormInput id="name" label="Nom" form_type="text" required={true}/>
                            <FormInput id="date" label="Date" form_type="date" required={true}/>
                            <FormInput id="location" label="Lieu" form_type="text" required={true}/>
                            <FormInput id="description" label="Description" form_type="text" required={true}/>
                            <FormInput id="nb_team_per_group" label="Nombre d'équipes par groupe" form_type="text" required={true}/>
                            <FormInput id="phase_qualifications" label="Phase de qualifications" form_type="checkbox" disabled={true} required={false}/>
                            <FormInput id="phase_eliminations" label="Phase d'éliminations" form_type="checkbox" disabled={true} required={false}/>
                        </div>
                        <div class="w-1/2 m-4">
                            <ContextProvider<UseStateHandle<Vec<Team>>> context={teams.clone()}>
                                <Teams/>
                            </ContextProvider<UseStateHandle<Vec<Team>>>>
                        </div>
                    </div>
                <hr/>
                <h2>{"Phase de qualifications"}</h2>
                <ContextProvider<UseStateHandle<Vec<Group>>> context={groups.clone()}>
                    <Groups on_create={on_create_group_click}/>
                </ContextProvider<UseStateHandle<Vec<Group>>>>
                <QualificationPhase/>
                <hr/>
                <h2>{"Phase d'éliminations"}</h2>
                /*<Bracket/>*/
            </div>
        </HomeLayout>
    }
}