use yew::prelude::*;
use crate::api::models::Team;

#[derive(PartialEq, Properties)]
pub struct TeamNoDragProps {
    pub team: Team,
}

#[function_component]
pub fn TeamNoDrag(props: &TeamNoDragProps) -> Html {
    let TeamNoDragProps { team } = props;

    html! {
        <div
            class={classes!("p-2", "bg-nutLight", "rounded")}
        >
            <div class={""}>
                {&team.name}
            </div>
        </div>
    }
}
