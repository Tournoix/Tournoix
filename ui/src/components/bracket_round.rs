use std::collections::BTreeMap;

use yew::prelude::*;

use crate::{api::models::GameWithTeams, components::bracket_match::BracketMatch};

#[derive(PartialEq, Properties)]
pub struct BracketRoundProps {
    pub games: BTreeMap<i32, GameWithTeams>,
    pub round_id: i32,
    pub editable: bool,
    #[prop_or_default]
    pub on_game_update: Callback<i32>,
}

#[function_component]
pub fn BracketRound(props: &BracketRoundProps) -> Html {
    let BracketRoundProps {
        games,
        round_id,
        editable,
        on_game_update,
    } = props;
    let round_title = format!("Round {}", round_id);

    html! {
        <ul class={" round"}>
            <li class={"spacer"}>
                <div class={"bg-nutLight text-center border"}>{&round_title}</div>
                {"\u{00a0}"}
            </li>
            {games.iter().map(|(_place, game)| {
                html!(
                    <BracketMatch game={game.clone()} editable={editable} on_game_update={on_game_update} />
                )
            }).collect::<Html>()}
            <li class={"spacer"}>{"\u{00a0}"}</li>
        </ul>
    }
}
