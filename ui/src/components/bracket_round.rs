use yew::prelude::*;

use crate::{components::bracket_match::BracketMatch, api::models::GameWithTeams};

use super::bracket::Match;

#[derive(PartialEq, Properties)]
pub struct BracketRoundProps {
    pub games: Vec<GameWithTeams>,
    pub round_id: i32,
}

#[function_component]
pub fn BracketRound(props: &BracketRoundProps) -> Html {
    let BracketRoundProps {
        games,
        round_id
    } = props;
    let round_title = format!("Round {}", round_id);

    html! {
        <ul class={" round"}>
            <li class={"spacer"}>
                <div class={"bg-nutLight text-center border"}>{&round_title}</div>
                {"\u{00a0}"}
            </li>
            {games.iter().map(|game| {
                html!(
                    <BracketMatch game={game.clone()} />
                )
            }).collect::<Html>()}
            <li class={"spacer"}>{"\u{00a0}"}</li>
        </ul>
    }
}
