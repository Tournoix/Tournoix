use yew::prelude::*;

use crate::components::bracket_match::BracketMatch;

use super::bracket::Match;

#[derive(PartialEq, Properties)]
pub struct BracketRoundProps {
    pub matches: Vec<Match>,
    pub round_id: u32
}

#[function_component]
pub fn BracketRound(props: &BracketRoundProps) -> Html {
    let BracketRoundProps {matches, round_id} = props;
    let round_title = format!("Round {}", round_id+1);
    let mut match_id = 0;

    html! {
        <ul class={" round"}>
            {matches.iter().map(|game| {
                match_id += 1;
                html!(
                    <>
                        <li class={"spacer"}>
                            {if match_id == 1 {
                                html!(
                                    <div class={"bg-nutLight text-center border"}>{&round_title}</div>
                                )
                            } else {
                                html!()
                            }}
                            {"\u{00a0}"}
                        </li>
                        <BracketMatch game={game.clone()} />
                    </>
                )
            }).collect::<Html>()}
            <li class={"spacer"}>{"\u{00a0}"}</li>
        </ul>
    }
}