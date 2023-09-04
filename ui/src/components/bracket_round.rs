use yew::prelude::*;

use crate::components::bracket_match::BracketMatch;

use super::bracket::Match;

#[derive(PartialEq, Properties)]
pub struct BracketRoundProps {
    pub matches: Vec<Match>,
    pub round_id: u32,
    pub on_started_click: Option<Callback<i32>>,
    pub on_finished_click: Option<Callback<i32>>,
    pub on_score1_change: Option<Callback<(i32, i32)>>,
    pub on_score2_change: Option<Callback<(i32, i32)>>,
}

#[function_component]
pub fn BracketRound(props: &BracketRoundProps) -> Html {
    let BracketRoundProps {matches, round_id, on_started_click, on_finished_click, on_score1_change, on_score2_change} = props;
    let round_title = format!("Round {}", round_id);
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
                        <BracketMatch game={game.clone()} on_started_click={on_started_click} on_finished_click={on_finished_click} on_score1_change={on_score1_change} on_score2_change={on_score2_change} />
                    </>
                )
            }).collect::<Html>()}
            <li class={"spacer"}>{"\u{00a0}"}</li>
        </ul>
    }
}