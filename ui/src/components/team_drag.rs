use js_sys::Array;
use log::info;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;
use yew_hooks::{use_drag_with_options, UseDragOptions};

use crate::api::{
    self,
    models::{Team, TeamUpdate},
};

#[derive(PartialEq, Properties)]
pub struct TeamDragProps {
    pub team: Team,
    pub update_trigger: UseStateHandle<bool>,
}

#[function_component]
pub fn TeamDrag(props: &TeamDragProps) -> Html {
    let TeamDragProps {
        team,
        update_trigger
    } = props;
    let node = use_node_ref();
    let state = {
        let team_id = team.id.to_string();

        use_drag_with_options(
            node.clone(),
            UseDragOptions {
                ondragstart: Some(Box::new(move |e| {
                    info!("Drag start");
                    let _ = e.data_transfer().unwrap().set_data("team_id", &team_id);
                })),
                ..Default::default()
            },
        )
    };

    let dragging = use_state(|| false);
    let drag_pos_origin = use_state(|| (0, 0));
    let drag_pos = use_state(|| (0, 0));

    // Drag and drop for mobile (oskour)
    let on_touch_start = {
        let drag_pos_origin = drag_pos_origin.clone();
        let dragging = dragging.clone();

        Callback::from(move |e: TouchEvent| {
            info!("Touch start");

            dragging.set(true);
            drag_pos_origin.set((
                e.touches().get(0).unwrap().client_x(),
                e.touches().get(0).unwrap().client_y(),
            ));
        })
    };

    let on_touch_move = {
        let drag_pos = drag_pos.clone();

        Callback::from(move |e: TouchEvent| {
            info!("Touch move");
            e.prevent_default();

            let touch = e.touches().get(0).unwrap();
            drag_pos.set((touch.client_x(), touch.client_y()));

            let document = window().unwrap().document().unwrap();
            let groups = document.get_elements_by_class_name("group-item");

            for i in 0..groups.length() {
                let group = groups.item(i).unwrap();
                let group_rect = group.get_bounding_client_rect();

                let in_bound = !(touch.client_x() < group_rect.left() as i32
                    || touch.client_x() > group_rect.right() as i32
                    || touch.client_y() < group_rect.top() as i32
                    || touch.client_y() > group_rect.bottom() as i32);

                let arr = Array::new_with_length(1);
                arr.set(0, JsValue::from_str("group-hover"));

                if in_bound {
                    let _ = group.class_list().add(&arr);
                } else {
                    let _ = group.class_list().remove(&arr);
                }
            }
        })
    };

    let on_touch_end = {
        let dragging = dragging.clone();
        let drag_pos = drag_pos.clone();
        let update_trigger = update_trigger.clone();
        let team_id = team.id;

        Callback::from(move |_e: TouchEvent| {
            let document = window().unwrap().document().unwrap();
            info!("Touch end");
            dragging.set(false);

            let groups = document.get_elements_by_class_name("group-item");

            for i in 0..groups.length() {
                let group = groups.item(i).unwrap();
                let group_rect = group.get_bounding_client_rect();

                let in_bound = !(drag_pos.0 < group_rect.left() as i32
                    || drag_pos.0 > group_rect.right() as i32
                    || drag_pos.1 < group_rect.top() as i32
                    || drag_pos.1 > group_rect.bottom() as i32);

                let arr = Array::new_with_length(1);
                arr.set(0, JsValue::from_str("group-hover"));
                let _ = group.class_list().remove(&arr);

                if in_bound {
                    info!("Dropped to group: {}", group.id());
                    let update_trigger = update_trigger.clone();

                    spawn_local(async move {
                        let _ = api::teams::update(
                            team_id,
                            TeamUpdate {
                                name: None,
                                group: Some(group.id().parse::<i32>().unwrap()),
                            },
                        )
                        .await;

                        update_trigger.set(!*update_trigger);
                    });

                    break;
                }
            }
        })
    };

    html! {
        <div
            ontouchstart={on_touch_start}
            ontouchmove={on_touch_move}
            ontouchend={on_touch_end}
            class={classes!("p-2", "bg-nutLight", "rounded", "cursor-grab", if *state.dragging {"opacity-50"} else {""}, if *dragging {"z-50"} else {""})}
            ref={node}
            style={format!("touch-action: none; {}", if *dragging {format!("transform: translate({}px, {}px);", drag_pos.0 - drag_pos_origin.0, drag_pos.1  - drag_pos_origin.1)} else {"".to_string()})}
        >
            <div class={""}>
                {&team.name}
            </div>
        </div>
    }
}
