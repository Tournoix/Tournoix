use yew::prelude::*;
use yew_hooks::{use_drag_with_options, UseDragOptions};

use crate::api::models::Team;

#[derive(PartialEq, Properties)]
pub struct TeamDragProps {
    pub team: Team,
}

#[function_component]
pub fn TeamDrag(props: &TeamDragProps) -> Html {
    let TeamDragProps { team } = props;
    let node = use_node_ref();
    let state = {
        let team_id = team.id.to_string();
        use_drag_with_options(
            node.clone(),
            UseDragOptions {
                ondragstart: Some(Box::new(move |e| {
                    let _ = e.data_transfer().unwrap().set_data("team_id", &team_id);
                })),
                ..Default::default()
            },
        )
    };

    html! {
        <div class={format!("w-full p-2 bg-nutLight rounded cursor-grab {}", if *state.dragging {"opacity-50"} else {""})} ref={node}>
            <div class={""}>
                {&team.name}
            </div>
        </div>
    }
}
