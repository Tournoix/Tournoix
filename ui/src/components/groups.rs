use std::collections::BTreeMap;

use log::info;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::{use_drop_with_options, UseDropOptions};

use crate::{
    api::{
        self,
        models::{Team, TeamUpdate, Tournament},
    },
    components::{group_drop::GroupDrop, loading_circle::LoadingCircle, team_drag::TeamDrag},
};

#[derive(PartialEq, Clone)]
pub struct Group; // TODO add fields ?

#[derive(PartialEq, Properties)]
pub struct GroupsProps {
    pub tournament: Tournament,
    pub should_update: Option<UseStateHandle<bool>>
}

#[function_component]
pub fn Groups(props: &GroupsProps) -> Html {
    let GroupsProps { tournament, should_update } = props;

    let groups: UseStateHandle<BTreeMap<i32, Vec<Team>>> = use_state(|| BTreeMap::new());
    let trigger = use_state(|| false);
    let loading = use_state(|| true);

    let new_group_node = use_node_ref();
    let new_group_state = {
        let groups = groups.clone();
        let trigger = trigger.clone();
        use_drop_with_options(
            new_group_node.clone(),
            UseDropOptions {
                ondrop: Some(Box::new(move |e| {
                    let team_id = e
                        .data_transfer()
                        .unwrap()
                        .get_data("team_id")
                        .unwrap()
                        .parse::<i32>()
                        .unwrap();

                    let groups = groups.clone();
                    let trigger = trigger.clone();
                    spawn_local(async move {
                        let _ = api::teams::update(
                            team_id,
                            TeamUpdate {
                                name: None,
                                group: Some(groups.keys().max().unwrap() + 1),
                            },
                        )
                        .await;

                        trigger.set(!*trigger);
                    });
                })),
                ..Default::default()
            },
        )
    };

    let no_group_node = use_node_ref();
    let no_group_state = {
        let trigger = trigger.clone();
        use_drop_with_options(
            no_group_node.clone(),
            UseDropOptions {
                ondrop: Some(Box::new(move |e| {
                    let team_id = e
                        .data_transfer()
                        .unwrap()
                        .get_data("team_id")
                        .unwrap()
                        .parse::<i32>()
                        .unwrap();

                    let trigger = trigger.clone();
                    spawn_local(async move {
                        let _ = api::teams::update(
                            team_id,
                            TeamUpdate {
                                name: None,
                                group: Some(0),
                            },
                        )
                        .await;

                        trigger.set(!*trigger);
                    });
                })),
                ..Default::default()
            },
        )
    };

    {
        let tournament = tournament.clone();
        let groups = groups.clone();
        let loading = loading.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if let Some(teams) = tournament.get_teams().await.ok() {
                        let mut new_groups: BTreeMap<i32, Vec<Team>> = BTreeMap::new();
                        new_groups.insert(0, vec![]);

                        for team in teams {
                            info!("{:?}", team.group as usize);
                            if new_groups.contains_key(&team.group) {
                                new_groups.get_mut(&team.group).unwrap().push(team);
                            } else {
                                new_groups.insert(team.group, vec![team]);
                            }
                        }


                        info!("{:?}", new_groups);
                        groups.set(new_groups);
                        loading.set(false);
                    }
                });
            },
            (trigger.clone(), should_update.clone()),
        );
    }

    // let on_delete_click = |id: usize| Callback::from(move |_| {});

    html! {
        <div class="flex flex-col items-center p-3 w-full">
            <h3>{"Groupes"}</h3>
            <ul class="flex flex-wrap gap-3 w-full justify-center">
                <li id={"0"} class={format!("group-item {}", if *no_group_state.over {"bg-green-200"} else {"bg-nutLighter"} )} ref={no_group_node}>
                    <div class="flex justify-center">
                        <div>{"Dans aucun groupe"}</div>
                    </div>
                    <div class="flex flex-col gap-1  team-list">
                        if *loading {
                            <LoadingCircle />
                        } else {
                            {
                                groups.get(&0).unwrap().iter().map(|team| {
                                    html! (<TeamDrag team={team.clone()} update_trigger={trigger.clone()} />)
                                }).collect::<Html>()
                            }
                        }
                    </div>
                </li>
                    if *loading {
                        <LoadingCircle />
                    } else {
                    {
                        groups.iter().map(|(index, teams)| {
                            if *index == 0 {return html!(<></>)}
                            html!{
                                <GroupDrop id={index} teams={teams.clone()} update_trigger={trigger.clone()} />
                            }
                        }).collect::<Html>()
                    }
                }

                <li id={(groups.keys().max().unwrap_or(&0) + 1).to_string()} class={format!("group-item {}", if *new_group_state.over {"bg-green-200"} else {"bg-nutLighter"} )} ref={new_group_node}>
                    <img src="/img/plus.svg" class="add-btn"/>
                    {"Créer un groupe"}
                </li>
            </ul>
        </div>
    }
}
