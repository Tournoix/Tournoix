use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct QualificationPhaseProps {}

#[function_component]
pub fn QualificationPhase(props: &QualificationPhaseProps) -> Html {
    let QualificationPhaseProps {} = props;

    html! {
        <div class="bg-red-200">
            {"QualificationPhase component"}
            <div class="text-red-500">{"AFFICHER QUE SI CE TOURNOI A UNE PHASE DE QUALIFICATION"}</div>
        </div>
    }
}