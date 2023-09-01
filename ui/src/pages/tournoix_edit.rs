use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, window};
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{layouts::homelayout::HomeLayout, components::{backlink::Backlink, bracket::Bracket, qualificationPhase::QualificationPhase, groups::{Groups, Group}, button::Button, teams::{Teams, Team}, form_input::FormInput, join_code::JoinCode, notification::NotifType}, routers::Route, utils::utils::{fetch_tournament, add_delayed_notif}};

#[derive(PartialEq, Properties)]
pub struct TournoixEditProps {
    pub id: i32,
}

#[function_component]
pub fn TournoixEdit(props: &TournoixEditProps) -> Html {
    let TournoixEditProps { id } = props;
    let navigator = use_navigator().unwrap();

    let tournament = use_state(|| fetch_tournament(*id));

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

    let groups: UseStateHandle<Vec<Group>> = use_state(|| vec![
        Group { },
        Group { },
        Group { },
        Group { },
        Group { },
        Group { },
    ]);

    let on_edit_click = {
        let navigator = navigator.clone();
        let id = id.clone();
        let tournament = tournament.clone();

        Callback::from(move |_| {
            add_delayed_notif("Modification réussie", &format!("Vous avez modifié avec succès le tournoi \"{}\"", tournament.name), NotifType::Success);
            
            navigator.push(&Route::TournoixView{ id });
        })
    };
    
    html! {
        <HomeLayout>
            <div class="flex flex-col items-center h-full pb-16 pt-12 sm:w-9/12 w-11/12 mx-auto relative">
                <Backlink route={Route::Tournoix} label="Retour à la liste des tournoix"/>
                <h1 class="mb-5">{format!("Modification de \"{}\"", {tournament.name.to_string()})}</h1>
                <a onclick={on_click_view} class="a_link mb-6">{"Voir ce tournoi en mode affichage"}</a>
                <JoinCode code={tournament.code.to_string()}/>
                <hr/>
                <h2>{"Informations"}</h2>
                <div class="flex flex-row w-full justify-center gap-5 lg:flex-nowrap flex-wrap">
                    <div class="w-1/2">
                        <FormInput id="name" label="Nom" form_type="text" required={true}/>
                        <FormInput id="date" label="Date" form_type="date" required={true}/>
                        <FormInput id="location" label="Lieu" form_type="text" required={true}/>
                        <FormInput id="description" label="Description" form_type="text" required={true}/>
                        <FormInput id="nb_team_per_group" label="Nombre d'équipes par groupe" form_type="text" required={true}/>
                        <FormInput id="phase_qualifications" label="Phase de qualifications" form_type="checkbox" checked={tournament.is_qualif} disabled={true} required={false}/>
                        <FormInput id="phase_eliminations" label="Phase d'éliminations" form_type="checkbox" checked={tournament.is_elim} disabled={true} required={false}/>
                    </div>
                    <div class="w-1/2 m-4">
                        <ContextProvider<UseStateHandle<Vec<Team>>> context={teams.clone()}>
                            <Teams on_edit={on_edit_team_click}/>
                        </ContextProvider<UseStateHandle<Vec<Team>>>>
                    </div>
                </div>
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
                <Button class="sm:text-xl text-lg px-3 py-2 mx-auto mt-3 mb-16 hover:scale-110 bg-green-700" onclick={on_edit_click}>{"Modifier le tournoi"}</Button>
            </div>
        </HomeLayout>
    }
}