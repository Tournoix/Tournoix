use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ResultsProps {}

#[function_component]
pub fn Results(props: &ResultsProps) -> Html {
    let ResultsProps {} = props;

    html! {
        <div class="bg-green-200">
            {"Results component"}
            <div class="text-red-500">{"AFFICHER UNIQUEMENT SI TOUT LES MATCHS DE CE TOURNOIS SONT FERMÉS"}</div>
        </div>
    }
}