use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct EliminationPhaseProps {}

#[function_component]
pub fn EliminationPhase(props: &EliminationPhaseProps) -> Html {
    let EliminationPhaseProps {} = props;

    html! {
        <div class="bg-blue-200">
            {"EliminationPhase component"}
            <div class="text-red-500">{"AFFICHER QUE SI CE TOURNOI A UNE PHASE D'ELIMINATIONS"}</div>
        </div>
    }
}