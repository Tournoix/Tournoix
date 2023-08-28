use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct TournamentsProps {}

#[function_component]
pub fn Tournaments(props: &TournamentsProps) -> Html {
    let TournamentsProps {} = props;
    let names = vec!["Z-event", "LAN Leco 2023", "Pétanque FVJC"];

    html! {
        <ul class="flex gap-5">
            <li class="tournament-card">{"Ajouter un tournoi"}</li>
            {
                names.into_iter().map(|name| {
                    html!{<li class="tournament-card" key={name}>{ name }</li>}
                }).collect::<Html>()
            }
        </ul>
    }
}