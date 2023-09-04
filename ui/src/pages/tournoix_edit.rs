use std::str::FromStr;

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, HtmlInputElement};
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yew_router::prelude::use_navigator;

use crate::{
    api::{self, models::Tournament, tournoix::UpdateTournoixRequest},
    components::{
        backlink::Backlink,
        button::Button,
        form_input::FormInput,
        groups::{Group, Groups},
        join_code::JoinCode,
        notification::NotifType,
        qualification_phase::QualificationPhase,
        teams::{Team, Teams}, loading_circle::LoadingCircle,
    },
    layouts::homelayout::HomeLayout,
    routers::Route,
    utils::utils::add_delayed_notif,
};

#[derive(PartialEq, Properties)]
pub struct TournoixEditProps {
    pub id: i32,
}

#[function_component]
pub fn TournoixEdit(props: &TournoixEditProps) -> Html {
    let TournoixEditProps { id } = props;
    let navigator = use_navigator().unwrap();
    let tournament: UseStateHandle<Option<Tournament>> = use_state(|| None);
    let loading = use_state(|| true);

    // Form inputs
    let name_ref = use_node_ref();
    let date_ref = use_node_ref();
    let location_ref = use_node_ref();
    let description_ref = use_node_ref();
    let groupe_size_ref = use_node_ref();
    let qualif_ref = use_node_ref();
    let elim_ref = use_node_ref();


    {
        let tournament = tournament.clone();
        let loading = loading.clone();
        let id = id.clone();

        use_effect_once(move || {
            spawn_local(async move {
                tournament.set(api::tournoix::get(id).await.ok());
                loading.set(false);
            });
    
            || ()
        });
    }

    let teams: UseStateHandle<Vec<Team>> = use_state(|| {
        vec![
            Team {
                id: 0,
                is_being_edited: false,
                name: "Cloud9".to_string(),
            },
            Team {
                id: 1,
                is_being_edited: false,
                name: "FaZe Clan".to_string(),
            },
            Team {
                id: 2,
                is_being_edited: false,
                name: "NaVi".to_string(),
            },
            Team {
                id: 3,
                is_being_edited: false,
                name: "NRG Esports".to_string(),
            },
            Team {
                id: 4,
                is_being_edited: false,
                name: "G2 Esports".to_string(),
            },
            Team {
                id: 5,
                is_being_edited: false,
                name: "fnatic".to_string(),
            },
            Team {
                id: 6,
                is_being_edited: false,
                name: "Team with a comically long name".to_string(),
            },
            Team {
                id: 7,
                is_being_edited: false,
                name: "Team 42".to_string(),
            },
        ]
    });

    let on_click_view = {
        let navigator = navigator.clone();
        let id = id.clone();
        Callback::from(move |_| navigator.push(&Route::TournoixView { id }))
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
                    let input_element = document
                        .get_element_by_id(format!("input-team-{}", id).as_str())
                        .unwrap();
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

    let groups: UseStateHandle<Vec<Group>> =
        use_state(|| vec![Group {}, Group {}, Group {}, Group {}, Group {}, Group {}]);

    let on_submit = {
        let tournament = tournament.clone();

        let name_ref = name_ref.clone();
        let date_ref = date_ref.clone();
        let location_ref = location_ref.clone();
        let description_ref = description_ref.clone();
        let groupe_size_ref = groupe_size_ref.clone();
        // let qualif_ref = qualif_ref.clone();
        // let elim_ref = elim_ref.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let name = name_ref.cast::<HtmlInputElement>().unwrap().value();
            let date = date_ref.cast::<HtmlInputElement>().unwrap().value();
            let location = location_ref.cast::<HtmlInputElement>().unwrap().value();
            let description = description_ref.cast::<HtmlInputElement>().unwrap().value();
            let groupe_size = groupe_size_ref.cast::<HtmlInputElement>().unwrap().value();
            // let qualif = qualif_ref.cast::<HtmlInputElement>().unwrap().checked();
            // let elim = elim_ref.cast::<HtmlInputElement>().unwrap().checked();

            let groupe_size = match groupe_size.is_empty() {
                true => None,
                false => Some(groupe_size.parse().unwrap())
            };

            let date = chrono::NaiveDateTime::from_str(&format!("{}:00", date)).unwrap();

            let update_request = UpdateTournoixRequest {
                name: Some(name),
                date: Some(date),
                location: Some(location),
                description: Some(description),
                phase: None,
                size_group: groupe_size,
            };

            {
                let tournament = tournament.clone();
                spawn_local(async move {
                    match tournament.as_ref().unwrap().update(update_request).await {
                        Ok(tournament) => {
                            add_delayed_notif(
                                "Modification réussie",
                                &format!(
                                    "Vous avez modifié avec succès le tournoi \"{}\"",
                                    tournament.name
                                ),
                                NotifType::Success,
                            );
                        },

                        Err(e) => {
                            add_delayed_notif(
                                &format!("Erreur: {}", e.error.reason),
                                &e.error.description,
                                NotifType::Error,
                            );
                        }
                    }

                    // Reload page to update state (berk)
                    window().unwrap().location().reload().unwrap();
                });
            }
        })
    };

    html! {
        <HomeLayout>
            <div class="flex flex-col items-center h-full pb-16 pt-12 sm:w-9/12 w-11/12 mx-auto relative">
                <Backlink route={Route::Tournoix} label="Retour à la liste des tournoix"/>
                if *loading {
                    <LoadingCircle />
                } else {
                    if let Some(tournament) = &*tournament {
                        <h1 class="mb-5">{format!("Modification de \"{}\"", {tournament.name.to_string()})}</h1>
                        <a onclick={on_click_view} class="a_link mb-6">{"Voir ce tournoi en mode affichage"}</a>
                        <JoinCode code={tournament.code.to_string()}/>
                        <hr/>
                        <form class="flex flex-col items-center" onsubmit={on_submit}>
                            <h2>{"Informations"}</h2>
                            <div class="flex flex-row w-full justify-center gap-5 lg:flex-nowrap flex-wrap">
                                <div class="w-1/2">
                                    <FormInput id="name" label="Nom" form_type="text" value={tournament.name.clone()} _ref={name_ref} required={true}/>
                                    <FormInput id="date" label="Date" form_type="datetime-local" value={tournament.date_locale().format("%Y-%m-%dT%H:%M").to_string()}  _ref={date_ref} required={true}/>
                                    <FormInput id="location" label="Lieu" form_type="text" value={tournament.location.as_ref().unwrap_or(&String::new()).to_string()}  _ref={location_ref} required={true}/>
                                    <FormInput id="description" label="Description" form_type="text" value={tournament.description.clone()}  _ref={description_ref} required={true}/>
                                    <FormInput id="nb_team_per_group" label="Nombre d'équipes par groupe" form_type="number" value={if let Some(s) = tournament.size_group {s.to_string()} else {String::new()}}  _ref={groupe_size_ref}/>
                                    <FormInput id="phase_qualifications" label="Phase de qualifications" form_type="checkbox" checked={tournament.is_qualif()}  _ref={qualif_ref} disabled={true}/>
                                    <FormInput id="phase_eliminations" label="Phase d'éliminations" form_type="checkbox" checked={tournament.is_elim()}  _ref={elim_ref} disabled={true}/>
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
                            <Button class="sm:text-xl text-lg px-3 py-2 mx-auto mt-3 mb-16 hover:scale-110 bg-green-700">{"Modifier le tournoi"}</Button>
                        </form>
                    } else {
                        <div>{"Oups, ce tournoi n'existe pas :("}</div>
                    }
                }
            </div>
        </HomeLayout>
    }
}
