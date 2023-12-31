use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::{use_drop_with_options, UseDropOptions};

use crate::{
    api::{
        self,
        models::{Team, TeamUpdate},
    },
    components::{team_drag::TeamDrag, team_no_drag::TeamNoDrag},
};

#[derive(PartialEq, Properties)]
pub struct GroupDropProps {
    pub id: i32,
    pub teams: Vec<Team>,
    pub update_trigger: UseStateHandle<bool>,
    #[prop_or_default]
    pub editable: bool
}

#[function_component]
pub fn GroupDrop(props: &GroupDropProps) -> Html {
    let GroupDropProps {
        id,
        teams,
        update_trigger,
        editable
    } = props;
    let node = use_node_ref();
    let state = {
        let id = id.clone();
        let update_trigger = update_trigger.clone();

        use_drop_with_options(
            node.clone(),
            UseDropOptions {
                ondrop: Some(Box::new(move |e| {
                    let team_id = e
                        .data_transfer()
                        .unwrap()
                        .get_data("team_id")
                        .unwrap()
                        .parse::<i32>()
                        .unwrap();

                    let update_trigger = update_trigger.clone();
                    spawn_local(async move {
                        let _ = api::teams::update(
                            team_id,
                            TeamUpdate {
                                name: None,
                                group: Some(id),
                            },
                        )
                        .await;
                        update_trigger.set(!*update_trigger);
                    });
                })),
                ..Default::default()
            },
        )
    };

    html! {
         <li id={id.to_string()} class={format!("group-item relative {}", if *state.over {"bg-green-200"} else {"bg-nutLighter"})} ref={node}>
            <div class="flex justify-center">
                <div>{"Groupe "}{id}</div>
            </div>
            <div class="flex flex-col gap-1 team-list">
                {
                    teams.iter().map(|team| {
                        if *editable {
                            html! (<TeamDrag team={team.clone()} update_trigger={update_trigger.clone()} />)
                        } else {
                            html! (<TeamNoDrag team={team.clone()} />)
                        }
                    }).collect::<Html>()
                }
            </div>
        </li>
    }
}
