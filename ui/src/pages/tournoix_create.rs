use wasm_bindgen::JsCast;
use web_sys::{console, window, HtmlInputElement};
use yew::prelude::*;

use crate::{layouts::homelayout::HomeLayout, components::{form_input::FormInput, button::Button, backlink::Backlink, teams::{Teams, Team, self}}};
use crate::routers::Route;

#[derive(PartialEq, Properties)]
pub struct TournoixCreateProps {
}

#[function_component]
pub fn TournoixCreate(props: &TournoixCreateProps) -> Html {
    let TournoixCreateProps {} = props;

    let teams: UseStateHandle<Vec<Team>> = use_state(|| vec![
        Team { id: 0, is_being_edited: false, name: "Cloud9".to_string() },
        Team { id: 1, is_being_edited: false, name: "FaZe Clan".to_string() },
        Team { id: 2, is_being_edited: false, name: "NaVi".to_string() },
        Team { id: 3, is_being_edited: false, name: "NRG Esports".to_string() },
        Team { id: 4, is_being_edited: false, name: "G2 Esports".to_string() },
        Team { id: 5, is_being_edited: false, name: "fnatic".to_string() },
        Team { id: 6, is_being_edited: false, name: "Team with a comically long name".to_string() }
    ]);

    let on_create_team_click = {
        let teams = teams.clone();
        Callback::from(move |_| {
            // Deep copy the teams vector into a buffer
            let mut teams_buf = vec![];
            for team in teams.iter() {
                let mut team = team.clone();
                team.is_being_edited = false;
                teams_buf.push(team);
            }

            teams_buf.push(Team {
                id: teams.len() as i32,
                is_being_edited: true,
                name: "Sans nom".to_string()
            });

            teams.set(teams_buf);
        })
    };

    let on_edit_team_click = {
        let teams = teams.clone();
        Callback::from(move |id| {
            // Deep copy the teams vector into a buffer
            let mut teams_buf = vec![];
            for team in teams.iter() {
                teams_buf.push(team.clone());
            }

            // Mark all other team as not being edited
            for team in teams_buf.iter_mut() {
                if team.id != id {
                    team.is_being_edited = false;
                }
            }
            
            let team_to_edit = teams_buf.iter_mut().find(|team| team.id == id);

            if let Some(team_to_edit) = team_to_edit {
                if team_to_edit.is_being_edited {
                    let mut team_name = "".to_string();

                    let window = window().unwrap();
                    let document = window.document().unwrap();
                    let input_element = document.get_element_by_id(format!("input-team-{}", id).as_str()).unwrap();
                    let input_element = input_element.dyn_into::<HtmlInputElement>().ok();
                    if let Some(input_element) = input_element {
                        team_name = input_element.value();
                    }
                    
                    team_to_edit.name = team_name;
                }

                team_to_edit.is_being_edited = !team_to_edit.is_being_edited;
            }
            
            teams.set(teams_buf);
        })
    };
    let on_delete_team_click = {
        let teams = teams.clone();
        Callback::from(move |id| {
            let mut name = "";

            // Deep copy the teams vector into a buffer
            let mut teams_buf = vec![];
            for team in teams.iter() {
                if team.id != id {
                    teams_buf.push(team.clone());
                } else {
                    name = team.name.as_str();
                }
            }

            if !gloo_dialogs::confirm(format!("Êtes vous sur de vouloir supprimer l'équipe \"{}\" ?", name).as_str()) {
                return;
            }
            
            teams.set(teams_buf);
        })
    };
    let on_create_click = Callback::from(move |_| { });
    
    html! {
        <HomeLayout>
            <div class="flex flex-col items-center h-full pb-16 pt-12 sm:w-9/12 w-11/12 mx-auto relative">
                <Backlink route={Route::Tournoix} label="Retour à la liste des tournoix"/>
                <h1 class="mb-5">{"Création de tournoi"}</h1>
                <form class="flex flex-col items-center w-full mx-auto relative">
                    <h2>{"Général"}</h2>
                    <div class="flex flex-row w-full justify-center gap-5 lg:flex-nowrap flex-wrap">
                        <div class="w-1/2">
                            <FormInput id="name" label="Nom" form_type="text" required={true}/>
                            <FormInput id="date" label="Date" form_type="date" required={true}/>
                            <FormInput id="location" label="Lieu" form_type="text" required={true}/>
                            <FormInput id="description" label="Description" form_type="text" required={true}/>
                            <FormInput id="nb_team_per_group" label="Nombre d'équipes par groupe" form_type="text" required={true}/>
                            <FormInput id="phase_qualifications" label="Phase de qualifications" form_type="checkbox" required={false}/>
                            <FormInput id="phase_eliminations" label="Phase d'éliminations" form_type="checkbox" required={false}/>
                        </div>
                        <div class="w-1/2 m-4">
                            <ContextProvider<UseStateHandle<Vec<Team>>> context={teams.clone()}>
                                <Teams on_create={on_create_team_click} on_edit={on_edit_team_click} on_delete={on_delete_team_click}/>
                            </ContextProvider<UseStateHandle<Vec<Team>>>>
                        </div>
                    </div>
                    <hr/>
                    <h2>{"Phase de qualifications"}</h2>
                    <hr/>
                    <h2>{"Phase d'éliminations"}</h2>
                    <hr/>
                    <Button class="sm:text-xl text-lg px-3 py-2 mx-auto mt-3 mb-16 hover:scale-110 bg-green-700" onclick={on_create_click}>{"Créer un tournoi"}</Button>
                </form>
            </div>
        </HomeLayout>
    }
}