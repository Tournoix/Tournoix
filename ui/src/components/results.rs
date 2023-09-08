use time::Duration;

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{utils::utils::team_color_wrapper, api::{self, tournoix::Score}, notification::{NotifType, CustomNotification}};

#[derive(PartialEq, Properties)]
pub struct ResultsProps {
    pub tournament_id: i32,
    pub can_show_results: bool,
}

#[function_component]
pub fn Results(props: &ResultsProps) -> Html {
    let ResultsProps { tournament_id, can_show_results } = props;
    let results = use_state(|| None);

    {
        let results = results.clone();
        let tournament_id = tournament_id.clone();

        use_effect_with_deps(move |_| {

            let results = results.clone();
            let tournament_id = tournament_id.clone();
            
            spawn_local(async move {
                let data = match api::tournoix::get_tournoix_results(tournament_id).await {
                    Ok(t) => Some(t),
                    Err(e) => {
                        None
                    }
                };

                if let Some(data) = data {
                    // Sort by score value
                    let mut teams_score_copy: Vec<Score> = vec![];
                    let mut gamblers_score_copy: Vec<Score> = vec![];
                    for score in (*data.teams).iter() {
                        teams_score_copy.push({
                            Score {
                                name: score.name.clone(),
                                val: score.val,
                            }
                        });
                    }
                    for score in (*data.subscribers).iter() {
                        gamblers_score_copy.push({
                            Score {
                                name: score.name.clone(),
                                val: score.val,
                            }
                        });
                    }

                    // Sort by rank, smaller is better
                    teams_score_copy.sort_by(|a, b| a.val.cmp(&b.val));

                    // Sort by nut number, bigger is better
                    gamblers_score_copy.sort_by(|a, b| b.val.cmp(&a.val));

                    results.set(Some(
                        api::tournoix::Results {
                            teams: teams_score_copy,
                            subscribers: gamblers_score_copy,
                        }
                    ));
                }
            });
        }, tournament_id.clone());
    }

    html! {
        <div class="flex">
            if *can_show_results {
                if let Some(results) = &*results {
                    <div class="ml-4 p-4 bg-nutLight">
                        <h3>{"Classement des parieurs"}</h3>
                        <ul class="h-96 overflow-y-scroll">
                            {
                                results.subscribers.iter().enumerate().map(|(index, score)| {
                                    html!{<li class="px-2 m-2 bg-nutLighter">
                                        {format!("{}) {} - {}", (index + 1).to_string(), score.name, score.val.to_string())}
                                    </li>}
                                }).collect::<Html>()
                            }
                        </ul>
                    </div>
                    <div class="flex flex-col gap-12">
                        <div class="flex flex-col justify-center items-center m-4">
                            if let Some(winning_gambler_score) = results.subscribers.get(0) {
                                <h1 class="mb-8">{"Bravo !"}</h1>
                                <img class="w-24 wiggle mb-8" src="/img/cup_first.png"/>
                                <h2>{winning_gambler_score.name.clone()}</h2>
                                <h2>{"a le plus de noix !"}</h2>
                                <div class="flex items-center">
                                    <span class="text-3xl mr-2">{winning_gambler_score.val.clone().to_string()}</span>
                                    <img class="w-14" src="/img/nut.svg"/>
                                </div>
                            }
                        </div>
                    </div>
                } else {
                    <div class="flex flex-col justify-center items-center">
                        <div class="mb-6">{"Les résultats ne sont pas encore disponibles"}</div>
                        <img class="w-24 wiggle" src="/img/question_mark.png"/>
                    </div>
                }
            } else {
                <div class="flex flex-col justify-center items-center">
                    <div class="mb-6">{"Les résultats ne sont pas encore disponibles"}</div>
                    <img class="w-24 wiggle" src="/img/question_mark.png"/>
                </div>
            }
        </div>
    }
}
