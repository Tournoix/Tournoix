use std::str::FromStr;

use log::info;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, HtmlInputElement};
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yew_router::prelude::use_navigator;

use crate::{
    api::{self, models::Tournament, tournoix::UpdateTournoixRequest, EmptyResponse},
    components::{
        backlink::Backlink,
        bracket::{Bracket, Match},
        button::Button,
        form_input::FormInput,
        groups::{Group, Groups},
        join_code::JoinCode,
        loading_circle::LoadingCircle,
        notification::NotifType,
        qualification_phase::QualificationPhase,
        teams::Teams,
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
    let is_qualif = use_state(|| false);
    let is_elim = use_state(|| false);

    info!("{}", serde_json::to_string(&EmptyResponse{}).unwrap());

    let on_qualif_change = {
        let is_qualif = is_qualif.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();

            is_qualif.set(target.unchecked_into::<HtmlInputElement>().checked());
        })
    };

    let on_elim_change = {
        let is_elim = is_elim.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();

            is_elim.set(target.unchecked_into::<HtmlInputElement>().checked());
        })
    };

    {
        let tournament = tournament.clone();
        let loading = loading.clone();
        let id = id.clone();

        use_effect_once(move || {
            spawn_local(async move {
                // TODO: fetch tournoix teams and matches
                tournament.set(api::tournoix::get(id).await.ok());
                loading.set(false);
            });

            || ()
        });
    }

    let on_click_view = {
        let navigator = navigator.clone();
        let id = id.clone();
        Callback::from(move |_| navigator.push(&Route::TournoixView { id }))
    };

    /*
    let on_edit_team_click = {
        let teams = teams.clone();
        Callback::from(move |id: i32| {
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
    */

    let groups: UseStateHandle<Vec<Group>> =
        use_state(|| vec![Group {}, Group {}, Group {}, Group {}, Group {}, Group {}]);

    let group_matches = use_state(|| {
        vec![
            vec![
                Match {
                    id: 0,
                    team1: "Cloud9".to_string(),
                    score1: 0,
                    team2: "FaZe Clan".to_string(),
                    score2: 0,
                    started: false,
                    finished: false,
                },
                Match {
                    id: 1,
                    team1: "NaVi".to_string(),
                    score1: 0,
                    team2: "NRG Esports".to_string(),
                    score2: 0,
                    started: true,
                    finished: false,
                },
                Match {
                    id: 2,
                    team1: "G2 Esports".to_string(),
                    score1: 0,
                    team2: "fnatic".to_string(),
                    score2: 0,
                    started: true,
                    finished: true,
                },
            ],
            vec![
                Match {
                    id: 3,
                    team1: "Team with a comically long name".to_string(),
                    score1: 0,
                    team2: "Team 42".to_string(),
                    score2: 0,
                    started: false,
                    finished: false,
                },
                Match {
                    id: 4,
                    team1: "TBA".to_string(),
                    score1: 0,
                    team2: "TBA".to_string(),
                    score2: 0,
                    started: false,
                    finished: false,
                },
            ],
        ]
    });

    let elim_matches = use_state(|| {
        vec![
            vec![
                Match {
                    id: 0,
                    team1: "Cloud9".to_string(),
                    score1: 0,
                    team2: "FaZe Clan".to_string(),
                    score2: 0,
                    started: false,
                    finished: false,
                },
                Match {
                    id: 1,
                    team1: "NaVi".to_string(),
                    score1: 0,
                    team2: "NRG Esports".to_string(),
                    score2: 0,
                    started: true,
                    finished: false,
                },
            ],
            vec![Match {
                id: 2,
                team1: "G2 Esports".to_string(),
                score1: 0,
                team2: "fnatic".to_string(),
                score2: 0,
                started: true,
                finished: true,
            }],
        ]
    });

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
                false => Some(groupe_size.parse().unwrap()),
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
                        }

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

    let on_create_group_click = {
        let groups = groups.clone();
        Callback::from(move |_| {
            // Deep copy the groups vector into a buffer
            let mut groups_buf = vec![];
            for group in groups.iter() {
                let group = group.clone();
                groups_buf.push(group);
            }

            groups_buf.push(Group { });

            groups.set(groups_buf);
        })
    };

    let on_delete_group_click = {
        let groups = groups.clone();
        Callback::from(move |index| {
            // Deep copy the groups vector into a buffer
            let mut groups_buf = vec![];
            for (_index, group) in groups.iter().enumerate() {
                if _index != index {
                    groups_buf.push(group.clone());
                } else {
                    // Check if the group is empty, otherwise cannot delete it
                }
            }
            
            groups.set(groups_buf);
        })
    };

    let on_started_click = |group_matches: UseStateHandle<Vec<Vec<Match>>>| {
        Callback::from(move |match_id| {
            // Deep copy the group_matches vector into a buffer
            let mut group_matches_buf = vec![];
            for group_match in group_matches.iter() {
                let mut _group_match = vec![];
                let group_match = group_match.clone();

                for mut _match in group_match.iter() {
                    let mut _match = _match.clone();

                    if match_id == _match.id {
                        _match.started = !_match.started;

                        // TODO DB
                        // UPDATE match SET started = _match.started WHERE id = _match.id
                    }

                    _group_match.push(_match);
                }

                group_matches_buf.push(_group_match);
            }

            group_matches.set(group_matches_buf);
        })
    };
    let on_finished_click = |group_matches: UseStateHandle<Vec<Vec<Match>>>| {
        Callback::from(move |match_id| {
            // Deep copy the group_matches vector into a buffer
            let mut group_matches_buf = vec![];
            for group_match in group_matches.iter() {
                let mut _group_match = vec![];
                let group_match = group_match.clone();

                for mut _match in group_match.iter() {
                    let mut _match = _match.clone();

                    if match_id == _match.id {
                        _match.finished = !_match.finished;
                    }

                    // TODO DB
                    // UPDATE match SET finished = _match.finished WHERE id = _match.id

                    _group_match.push(_match);
                }

                group_matches_buf.push(_group_match);
            }

            group_matches.set(group_matches_buf);
        })
    };
    let on_score1_change = |group_matches: UseStateHandle<Vec<Vec<Match>>>| {
        Callback::from(move |(match_id, val)| {
            // Deep copy the group_matches vector into a buffer
            let mut group_matches_buf = vec![];
            for group_match in group_matches.iter() {
                let mut _group_match = vec![];
                let group_match = group_match.clone();

                for mut _match in group_match.iter() {
                    let mut _match = _match.clone();

                    if match_id == _match.id {
                        _match.score1 = val;
                    }

                    // TODO DB
                    // UPDATE match SET score1 = _match.score1 WHERE id = _match.id

                    _group_match.push(_match);
                }

                group_matches_buf.push(_group_match);
            }

            group_matches.set(group_matches_buf);
        })
    };
    let on_score2_change = |group_matches: UseStateHandle<Vec<Vec<Match>>>| {
        Callback::from(move |(match_id, val)| {
            // Deep copy the group_matches vector into a buffer
            let mut group_matches_buf = vec![];
            for group_match in group_matches.iter() {
                let mut _group_match = vec![];
                let group_match = group_match.clone();

                for mut _match in group_match.iter() {
                    let mut _match = _match.clone();

                    if match_id == _match.id {
                        _match.score2 = val;
                    }

                    // TODO DB
                    // UPDATE match SET score2 = _match.score2 WHERE id = _match.id

                    _group_match.push(_match);
                }

                group_matches_buf.push(_group_match);
            }

            group_matches.set(group_matches_buf);
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
                        <h2>{"Informations"}</h2>
                        <div class="flex flex-row w-full justify-center gap-5 lg:flex-nowrap flex-wrap">
                            <div class="w-1/2">
                                <form class="flex flex-col items-end" onsubmit={on_submit}>
                                    <FormInput id="name" label="Nom" form_type="text" value={tournament.name.clone()} _ref={name_ref} required={true}/>
                                    <FormInput id="date" label="Date" form_type="datetime-local" value={tournament.date_locale().format("%Y-%m-%dT%H:%M").to_string()}  _ref={date_ref} required={true}/>
                                    <FormInput id="location" label="Lieu" form_type="text" value={tournament.location.as_ref().unwrap_or(&String::new()).to_string()}  _ref={location_ref} required={true}/>
                                    <FormInput id="description" label="Description" form_type="text" value={tournament.description.clone()}  _ref={description_ref} required={true}/>
                                    <FormInput id="nb_team_per_group" label="Nombre d'équipes par groupe" form_type="number" value={if let Some(s) = tournament.size_group {s.to_string()} else {String::new()}}  _ref={groupe_size_ref}/>
                                    <FormInput id="phase_qualifications" label="Phase de qualifications" form_type="checkbox" checked={*is_qualif} onchange={on_qualif_change} />
                                    <FormInput id="phase_eliminations" label="Phase d'éliminations" form_type="checkbox" checked={*is_elim} onchange={on_elim_change} />

                                    <Button class="text-lg px-3 py-2 mt-3 hover:scale-110 bg-green-700">{"Modifier les informations"}</Button>
                                </form>
                            </div>
                            <div class="w-1/2 m-4">
                                <Teams tournament={tournament.clone()} />
                                //<ContextProvider<UseStateHandle<Vec<Team>>> context={teams.clone()}>
                                    //
                                //</ContextProvider<UseStateHandle<Vec<Team>>>>
                            </div>
                        </div>
                        if *is_qualif {
                            <hr/>
                            <h2>{"Phase de qualifications"}</h2>
                            <ContextProvider<UseStateHandle<Vec<Group>>> context={groups.clone()}>
                                <Groups on_create={on_create_group_click} on_delete={on_delete_group_click}/>
                            </ContextProvider<UseStateHandle<Vec<Group>>>>
                            <ContextProvider<UseStateHandle<Vec<Vec<Match>>>> context={group_matches.clone()}>
                                <QualificationPhase on_started_click={on_started_click(group_matches.clone())} on_finished_click={on_finished_click(group_matches.clone())} on_score1_change={on_score1_change(group_matches.clone())} on_score2_change={on_score2_change(group_matches.clone())}/>
                            </ContextProvider<UseStateHandle<Vec<Vec<Match>>>>>
                        }
                        if *is_elim {
                            <hr/>
                            <h2>{"Phase d'éliminations"}</h2>
                            <Bracket teams={(*elim_matches).clone()} on_started_click={on_started_click(elim_matches.clone())} on_finished_click={on_finished_click(elim_matches.clone())} on_score1_change={on_score1_change(elim_matches.clone())} on_score2_change={on_score2_change(elim_matches.clone())} />
                        }
                        <hr/>
                    } else {
                        <div>{"Oups, ce tournoi n'existe pas :("}</div>
                    }
                }
            </div>
        </HomeLayout>
    }
}
