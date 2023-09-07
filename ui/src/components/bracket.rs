use std::collections::BTreeMap;

use crate::{
    api::models::{GameWithTeams, Team, Tournament},
    components::bracket_round::BracketRound,
};
use log::info;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Match {
    pub id: i32,
    pub team1: String,
    pub score1: i32,
    pub team2: String,
    pub score2: i32,
    pub started: bool,
    pub finished: bool,
}

#[derive(PartialEq, Properties)]
pub struct BracketProps {
    pub tournament: Tournament,
    pub should_update: UseStateHandle<bool>,
    pub editable: bool,
    /*
    pub teams: BracketTeams,
    pub on_started_click: Option<Callback<i32>>,
    pub on_finished_click: Option<Callback<i32>>,
    pub on_score1_change: Option<Callback<(i32, i32)>>,
    pub on_score2_change: Option<Callback<(i32, i32)>>,
    */
}

#[function_component]
pub fn Bracket(props: &BracketProps) -> Html {
    let BracketProps {
        tournament,
        should_update,
        editable,
    } = props;

    //let is_nb_valid = teams.len() > 0 && (teams[0].len() & (teams[0].len() - 1)) == 0;
    let teams: UseStateHandle<BTreeMap<i32, BTreeMap<i32, GameWithTeams>>> =
        use_state(|| BTreeMap::new());
    let is_nb_valid = true;
    let trigger = use_state(|| false);

    {
        let tournament = tournament.clone();
        let teams = teams.clone();
        // let loading = loading.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if let Some(games) = tournament.get_matches().await.ok() {
                        let mut new_teams: BTreeMap<i32, BTreeMap<i32, GameWithTeams>> =
                            BTreeMap::new();
                        // new_groups.insert(0, vec![]);

                        for game in games {
                            if game.phase < 1 {
                                continue;
                            }

                            new_teams
                                .entry(game.phase)
                                .or_insert_with(|| BTreeMap::new())
                                .entry(game.place)
                                .or_insert_with(|| game);

                            /*
                            if new_teams.contains_key(&game.phase) {
                                new_teams.get_mut(&game.phase).unwrap().push(game);
                            } else {
                                new_teams.insert(game.phase, vec![game]);
                            }
                            */
                        }

                        if new_teams.len() == 0 {
                            return;
                        }

                        let games_len = new_teams.get(&1).unwrap().len();

                        let mut phase_id = 2;
                        for i in (1..=games_len / 2).rev() {
                            if !new_teams.contains_key(&phase_id) {
                                new_teams.insert(phase_id, BTreeMap::new());
                            }

                            let phase = new_teams.get_mut(&phase_id).unwrap();
                            for j in 0..i {
                                if !phase.contains_key(&(j as i32)) {
                                    info!("insert");
                                    phase.insert(
                                        j as i32,
                                        GameWithTeams {
                                            id: -1,
                                            team1: Team {
                                                id: -1,
                                                name: "TBA".into(),
                                                fk_tournaments: -1,
                                                group: -1,
                                            },
                                            team2: Team {
                                                id: -1,
                                                name: "TBA".into(),
                                                fk_tournaments: -1,
                                                group: -1,
                                            },
                                            score1: 0,
                                            score2: 0,
                                            phase: phase_id,
                                            place: j as i32,
                                            status: 0,
                                            has_gained_nut: false,
                                            group: Some(-1),
                                        },
                                    );
                                }
                            }

                            phase_id += 1;
                        }

                        teams.set(new_teams);
                        // loading.set(false);
                    }
                });
            },
            (should_update.clone(), trigger.clone()),
        );
    }

    let on_game_update = Callback::from(move |_game_id: i32| {
        trigger.set(!*trigger);
    });

    if !is_nb_valid {
        html!(
            <div>{"Number of teams is invalid. Must be a power of 2"}</div>
        )
    } else {
        html! {
            <div class={"bracket"}>
                if teams.len() > 0 {
                    {teams.iter().map(|(round_id, games)| {
                        html!(
                            <BracketRound round_id={round_id} games={games.clone()} editable={editable} on_game_update={on_game_update.clone()} />
                        )
                    }).collect::<Html>()}
                    <div class="round">
                        <div class={"ml-4 p-1 font-bold text-center bg-nutLighter"}>{"Winner"}</div>
                    </div>
                } else {
                    {"Aucun matchs pour le moment"}
                }
            </div>
        }
    }
}
