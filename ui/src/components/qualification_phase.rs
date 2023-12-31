use std::collections::BTreeMap;

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{
    api::models::{GameWithTeams, Tournament},
    components::qualif_game::QualifGame,
};

#[derive(PartialEq, Properties)]
pub struct QualificationPhaseProps {
    pub tournament: Tournament,
    pub should_update: UseStateHandle<bool>,
    #[prop_or_default]
    pub editable: bool,
}

#[function_component]
pub fn QualificationPhase(props: &QualificationPhaseProps) -> Html {
    let QualificationPhaseProps {
        tournament,
        should_update,
        editable,
    } = props;

    let group_matches: UseStateHandle<BTreeMap<i32, Vec<GameWithTeams>>> =
        use_state(|| BTreeMap::new());
    let loading = use_state(|| true);
    let trigger = use_state(|| false);

    {
        let tournament = tournament.clone();
        let group_matches = group_matches.clone();
        let loading = loading.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if let Some(games) = tournament.get_matches().await.ok() {
                        let mut new_groups: BTreeMap<i32, Vec<GameWithTeams>> = BTreeMap::new();

                        for game in games {
                            if game.phase != 0 {
                                continue;
                            }

                            if game.group.unwrap() == 0 {
                                continue;
                            }
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
            (should_update.clone(), trigger.clone()),
        );
    }

    let on_game_update = Callback::from(move |_game_id: i32| {
        trigger.set(!*trigger);
    });

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
                                        html!{<QualifGame game={game.clone()} editable={*editable} on_game_update={on_game_update.clone()} />}
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
