use std::collections::BTreeMap;

use crate::{
    api::models::{GameWithTeams, Tournament},
    components::bracket_round::BracketRound,
};
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

pub type BracketTeams = Vec<Vec<Match>>;

#[derive(PartialEq, Properties)]
pub struct BracketProps {
    pub tournament: Tournament,
    pub should_update: UseStateHandle<bool>,
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
    let BracketProps { tournament, should_update } = props;

    //let is_nb_valid = teams.len() > 0 && (teams[0].len() & (teams[0].len() - 1)) == 0;
    let teams: UseStateHandle<BTreeMap<i32, Vec<GameWithTeams>>> = use_state(|| BTreeMap::new());
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
                        let mut new_teams: BTreeMap<i32, Vec<GameWithTeams>> = BTreeMap::new();
                        // new_groups.insert(0, vec![]);

                        for game in games {
                            if game.phase < 1 {
                                continue;
                            }

                            if new_teams.contains_key(&game.phase) {
                                new_teams.get_mut(&game.phase).unwrap().push(game);
                            } else {
                                new_teams.insert(game.phase, vec![game]);
                            }
                        }

                        teams.set(new_teams);
                        // loading.set(false);
                    }
                });
            },
            (should_update.clone(), trigger.clone()),
        );
    }

    if !is_nb_valid {
        html!(
            <div>{"Number of teams is invalid. Must be a power of 2"}</div>
        )
    } else {
        html! {
            <div class={"bracket"}>
                {teams.iter().map(|(round_id, games)| {
                    html!(
                        <BracketRound round_id={round_id} games={games.clone()} />
                    )
                }).collect::<Html>()}
                <div class="round">
                    <div class={"ml-4 p-1 font-bold text-center bg-nutLighter"}>{"Winner"}</div>
                </div>
            </div>
        }
    }
}
