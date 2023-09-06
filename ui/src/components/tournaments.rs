use yew::prelude::*;

use crate::{api::models::Tournament, components::tournament_card::TournamentCard};

#[derive(PartialEq, Properties)]
pub struct TournamentsProps {
    pub tournaments: Vec<Tournament>,
    #[prop_or_default]
    pub editable: bool,
    #[prop_or_default]
    pub on_delete: Callback<Tournament>,
    /*
    pub on_create: Option<Callback<MouseEvent>>,
    pub on_edit: Option<Callback<MouseEvent>>,
    pub nb_nuts: Option<i32>,
    pub on_leave: Option<Callback<MouseEvent>>,
    */
} // TODO: Stop passing function in props if functions are component specific

// TODO when invoking a on_... function, pass the id of the tournament to it

#[function_component]
pub fn Tournaments(props: &TournamentsProps) -> Html {
    let TournamentsProps {
        tournaments,
        editable,
        on_delete,
    } = props;

    html! {
        <ul class="flex gap-5 flex-wrap">
            // List tournaments
            {
                tournaments.iter().map(|tournament| {
                    html!{
                        <li>
                            <TournamentCard tournament={tournament.clone()} editable={editable} on_delete={on_delete} />
                        </li>
                    }
                }).collect::<Html>()
            }
        </ul>
    }
}
