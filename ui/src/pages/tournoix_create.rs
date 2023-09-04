use wasm_bindgen::JsCast;
use web_sys::{window, HtmlInputElement};
use yew::prelude::*;

use crate::{layouts::homelayout::HomeLayout, components::{form_input::FormInput, button::Button, backlink::Backlink, teams::{Teams, Team}, bracket::{Bracket, Match, BracketTeams}, groups::{Group, Groups}, checkbox::CheckBox, qualificationPhase::QualificationPhase}};
use crate::routers::Route;

#[derive(PartialEq, Properties)]
pub struct TournoixCreateProps {
}

#[function_component]
pub fn TournoixCreate(props: &TournoixCreateProps) -> Html {
    let TournoixCreateProps {} = props;

    let is_qualif = use_state(|| false);
    let is_elim = use_state(|| false);

    let on_elim_click = {
        let is_elim = is_elim.clone();
        Callback::from(move |_| {
            is_elim.set(!*is_elim);
        })
    };
    let on_qualif_click = {
        let is_qualif = is_qualif.clone();
        Callback::from(move |_| {
            is_qualif.set(!*is_qualif);
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

    let on_create_group_click = {
        let groups = groups.clone();
        Callback::from(move |_| {
            // Deep copy the groups vector into a buffer
            let mut groups_buf = vec![];
            for group in groups.iter() {
                let mut group = group.clone();
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

    let group_matches = use_state(|| vec![
        vec![
            Match {
                id: 0,
                team1: "Cloud9".to_string(),
                score1: 0,
                team2: "FaZe Clan".to_string(),
                score2: 0,
                started: false,
                finished: false
            },
            Match {
                id: 1,
                team1: "NaVi".to_string(),
                score1: 0,
                team2: "NRG Esports".to_string(),
                score2: 0,
                started: true,
                finished: false
            },
            Match {
                id: 2,
                team1: "G2 Esports".to_string(),
                score1: 0,
                team2: "fnatic".to_string(),
                score2: 0,
                started: true,
                finished: true
            },
        ],
        vec! [
            Match {
                id: 3,
                team1: "Team with a comically long name".to_string(),
                score1: 0,
                team2: "Team 42".to_string(),
                score2: 0,
                started: false,
                finished: false
            },
            Match {
                id: 4,
                team1: "TBA".to_string(),
                score1: 0,
                team2: "TBA".to_string(),
                score2: 0,
                started: false,
                finished: false
            },
        ]
    ]);

    let on_started_click = {
        let group_matches = group_matches.clone();
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
                    }

                    _group_match.push(_match);
                }

                group_matches_buf.push(_group_match);
            }

            group_matches.set(group_matches_buf);
        })
    };
    let on_finished_click = {
        let group_matches = group_matches.clone();
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

                    _group_match.push(_match);
                }

                group_matches_buf.push(_group_match);
            }

            group_matches.set(group_matches_buf);
        })
    };

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

            if !gloo_dialogs::confirm(format!("Êtes-vous sûr de vouloir supprimer l'équipe \"{}\" ?", name).as_str()) {
                return;
            }
            
            teams.set(teams_buf);
        })
    };

    let on_create_click = Callback::from(move |_| { });

    // Generate bracket
    let mut bracket_teams: BracketTeams = vec![];

    if teams.len() >= 2 && (teams.len() & (teams.len()-1)) == 0 {
        let nb_rounds = (teams.len() as f32).log2() as u32;

        bracket_teams.push((0..teams.len()).step_by(2).map(|i| {
            Match {
                id: 0,
                team1: teams[i].name.clone(),
                score1: 0,
                team2: teams[i+1].name.clone(),
                score2: 0,
                started: false,
                finished: false
            }
        }).collect::<Vec<Match>>());

        let mut nb_match = teams.len() / 4;
        for _i in 1..nb_rounds {
            bracket_teams.push((0..nb_match).map(|_| Match {
                id: 0,
                team1: "TBA".to_string(),
                score1: 0,
                team2: "TBA".to_string(),
                score2: 0,
                started: false,
                finished: false
            }).collect::<Vec<Match>>());

            nb_match /= 2;
        }
    }

    html! {
        <HomeLayout>
            <div class="flex flex-col items-center h-full pb-16 pt-12 sm:w-9/12 w-11/12 mx-auto relative">
                <Backlink route={Route::Tournoix} label="Retour à la liste des tournoix"/>
                <h1 class="mb-5">{"Création de tournoi"}</h1>
                <form class="flex flex-col items-center w-full mx-auto relative">
                    <h2>{"Informations"}</h2>
                    <div class="flex flex-row w-full justify-center gap-5 lg:flex-nowrap flex-wrap">
                        <div class="w-1/2">
                            <FormInput id="name" label="Nom" form_type="text" required={true}/>
                            <FormInput id="date" label="Date" form_type="date" required={true}/>
                            <FormInput id="location" label="Lieu" form_type="text" required={true}/>
                            <FormInput id="description" label="Description" form_type="text" required={true}/>
                            <FormInput id="nb_team_per_group" label="Nombre d'équipes par groupe" form_type="text" required={true}/>
                            <CheckBox id="phase_qualifications" label="Phase de qualifications" checked={*is_qualif} on_click={on_qualif_click}/>
                            <CheckBox id="phase_eliminations" label="Phase d'éliminations" checked={*is_elim} on_click={on_elim_click}/>
                        </div>
                        <div class="w-1/2 m-4">
                            <ContextProvider<UseStateHandle<Vec<Team>>> context={teams.clone()}>
                                <Teams on_create={on_create_team_click} on_edit={on_edit_team_click} on_delete={on_delete_team_click}/>
                            </ContextProvider<UseStateHandle<Vec<Team>>>>
                        </div>
                    </div>
                    if *is_qualif {
                        <hr/>
                        <h2>{"Phase de qualifications"}</h2>
                        <ContextProvider<UseStateHandle<Vec<Group>>> context={groups.clone()}>
                            <Groups on_create={on_create_group_click} on_delete={on_delete_group_click}/>
                        </ContextProvider<UseStateHandle<Vec<Group>>>>
                        <ContextProvider<UseStateHandle<Vec<Vec<Match>>>> context={group_matches.clone()}>
                            <QualificationPhase on_started_click={on_started_click} on_finished_click={on_finished_click}/>
                        </ContextProvider<UseStateHandle<Vec<Vec<Match>>>>>
                    }
                    if *is_elim {
                        <hr/>
                        <h2>{"Phase d'éliminations"}</h2>
                        <Bracket teams={bracket_teams} />
                    }
                    <hr/>
                    <Button disabled={!*is_qualif && !*is_elim} class="sm:text-xl text-lg px-3 py-2 mx-auto mt-3 mb-16 hover:scale-110 bg-green-700" onclick={on_create_click}>{"Créer un tournoi"}</Button>
                </form>
            </div>
        </HomeLayout>
    }
}