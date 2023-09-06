use std::collections::BTreeMap;

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

use crate::{
    api::models::{Tournament, GameWithTeams},
    components::{bracket::Match, checkbox::CheckBox},
    utils::utils::team_color_wrapper,
};

#[derive(PartialEq, Properties)]
pub struct QualificationPhaseProps {
    pub tournament: Tournament,
    pub should_update: UseStateHandle<bool>,
    #[prop_or_default]
    pub editable: bool,
    /*
    pub on_started_click: Option<Callback<i32>>,
    pub on_finished_click: Option<Callback<i32>>,
    pub on_score1_change: Option<Callback<(i32, i32)>>,
    pub on_score2_change: Option<Callback<(i32, i32)>>,
    */
}

#[function_component]
pub fn QualificationPhase(props: &QualificationPhaseProps) -> Html {
    let QualificationPhaseProps {
        tournament,
        should_update,
        editable, /*
                  on_started_click,
                  on_finished_click,
                  on_score1_change,
                  on_score2_change,
                  */
    } = props;

    let group_matches: UseStateHandle<BTreeMap<i32, Vec<GameWithTeams>>> = use_state(|| BTreeMap::new());
    let loading = use_state(|| true);

    {
        let tournament = tournament.clone();
        let group_matches = group_matches.clone();
        let loading = loading.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if let Some(games) = tournament.get_matches().await.ok() {
                        let mut new_groups: BTreeMap<i32, Vec<GameWithTeams>> = BTreeMap::new();
                        // new_groups.insert(0, vec![]);

                        for game in games {
                            if game.group.unwrap() == 0 {continue;}
                            if new_groups.contains_key(&game.group.unwrap()) {
                                new_groups.get_mut(&game.group.unwrap()).unwrap().push(game);
                            } else {
                                new_groups.insert(game.group.unwrap(), vec![game]);
                            }
                        }

                        group_matches.set(new_groups);
                        loading.set(false);
                    }
                });
            },
            should_update.clone(),
        );
    }
        
    let on_click_started = |id: i32| Callback::from(move |_| {});

    let on_click_finished = |id: i32| Callback::from(move |_| {});

    let change_score1 = |id| {
        Callback::from(move |e: Event| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");

            // You must KNOW target is a HtmlInputElement, otherwise
            // the call to value would be Undefined Behaviour (UB).
            // Here we are sure that this is input element so we can convert it to the appropriate type without checking
            if let Ok(val) = target
                .unchecked_into::<HtmlInputElement>()
                .value()
                .parse::<i32>()
            {}
        })
    };
    let change_score2 = |id| {
        Callback::from(move |e: Event| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");

            // You must KNOW target is a HtmlInputElement, otherwise
            // the call to value would be Undefined Behaviour (UB).
            // Here we are sure that this is input element so we can convert it to the appropriate type without checking
            if let Ok(val) = target
                .unchecked_into::<HtmlInputElement>()
                .value()
                .parse::<i32>()
            {}
        })
    };

    html! {
        <div class="w-full mt-4">
            <ul class="flex flex-wrap gap-3 justify-center items-center">
                {
                    group_matches.iter().map(|(index, group_match)| {
                        html!{<li class="rounded relative basis-72 bg-nutLighter flex flex-col justify-center items-center">
                            <h3 class="text-center">{"Groupe "}{index}</h3>
                            <ul>
                                {
                                    group_match.iter().map(|game| {
                                        html!{<div>
                                            <hr class="m-0 border-nutLight drop-shadow-none"/>
                                            <li class="rounded relative flex justify-center items-center">
                                                <div style={team_color_wrapper(game.team1.name.clone())} class="team-border-color border-r-4 px-2 m-2 rounded-l bg-nutLight w-24 text-right">
                                                    {game.team1.name.clone()}
                                                </div>
                                                <input type="number" value={game.score1.to_string()} disabled={!editable} onchange={(change_score1.clone())(game.id.clone())} class="mr-1 w-8 h-5 bg-white text-center" />
                                                {" - "}
                                                <input type="number" value={game.score2.to_string()} disabled={!editable} onchange={(change_score2.clone())(game.id.clone())} class="ml-1 w-8 h-5 bg-white text-center" />
                                                <div style={team_color_wrapper(game.team2.name.clone())} class="team-border-color border-l-4 px-2 m-2 rounded-r bg-nutLight w-24">
                                                    {game.team2.name.clone()}
                                                </div>
                                                <div class="flex flex-col mr-2 mb-1">
                                                    {
                                                        if game.status == 2 {
                                                            html!{<div class="font-bebas w-full text-xs rounded m-1 text-center text-white bg-green-600">{"TERMINÉ"}</div>}
                                                        } else if game.status == 1 {
                                                            html!{<div class="font-bebas w-full text-xs rounded m-1 text-center text-white bg-yellow-600">{"EN COURS"}</div>}
                                                        } else {
                                                            html!{<div class="font-bebas w-full text-xs rounded m-1 text-center text-white bg-orange-600">{"EN ATTENTE"}</div>}
                                                        }
                                                    }
                                                    if *editable {
                                                        <CheckBox class="m-0 text-xs" id={format!("started_{}", game.id.clone())} label="Started" checked={game.status == 1} on_click={on_click_started(game.id)}/>
                                                        <CheckBox class="m-0 text-xs" id={format!("finished_{}", game.id.clone())} label="Finished" checked={game.status == 2} on_click={on_click_finished(game.id)}/>
                                                    }
                                                </div>
                                            </li>
                                        </div>}
                                    }).collect::<Html>()
                                }
                            </ul>
                        </li>}
                    }).collect::<Html>()
                }
            </ul>
        </div>
    }
}
