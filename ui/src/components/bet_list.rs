use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{components::button::Button, routers::Route, utils::utils::team_color_wrapper, api::models::GameWithTeams};

#[derive(PartialEq, Properties)]
pub struct BetListProps {
    pub matches: Vec<GameWithTeams>,
    pub tournament_id: i32,
}

#[function_component]
pub fn BetList(props: &BetListProps) -> Html {
    let BetListProps { matches, tournament_id } = props;
    let navigator = use_navigator().unwrap();

    let on_click_bet = |match_id| {
        let tournament_id = tournament_id.clone();
        Callback::from(move |_| navigator.push(&Route::MatchView {
            match_id,
            tournament_id
        }))
    };

    html! {
        <ul class="flex gap-4 flex-wrap">{
            matches.iter().map(|_match| {
                html!{<li class="flex flex-col items-center rounded bg-nutLighter">
                    {
                        if _match.status == 2 {
                            html!{<div class="font-bebas rounded-t w-full mb-2 px-3 py-1 text-xs text-center text-white bg-green-600">{"TERMINÉ"}</div>}
                        } else if _match.status == 1 {
                            html!{<div class="font-bebas rounded-t w-full mb-2 px-3 py-1 text-xs text-center text-white bg-yellow-600">{"EN COURS"}</div>}
                        } else {
                            html!{<div class="font-bebas rounded-t w-full mb-2 px-3 py-1 text-xs text-center text-white bg-orange-600">{"EN ATTENTE"}</div>}
                        }
                    }
                    <div class="gap-2 flex items-center">
                        <span style={team_color_wrapper(_match.team1.name.clone())} class="team-text-color pl-3">{_match.team1.name.clone()}</span>
                        <img class="drop-shadow h-4" src="/img/versus_smol.png"/>
                        <span style={team_color_wrapper(_match.team2.name.clone())} class="team-text-color pr-3">{_match.team2.name.clone()}</span>
                    </div>
                    <Button class="mx-4 mb-3 py-1 px-3 mt-2 hover:scale-110" onclick={(on_click_bet.clone())(_match.id)}>{"Parier"}</Button>
                </li>}
            }).collect::<Html>()
        }</ul>
    }
}
