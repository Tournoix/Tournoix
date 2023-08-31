use yew::prelude::*;
use crate::components::bracket_round::BracketRound;

#[derive(PartialEq, Properties)]
pub struct BracketProps {
    pub nb_teams: usize
}

#[function_component]
pub fn Bracket(props: &BracketProps) -> Html {
    let BracketProps {nb_teams} = props;

    let is_nb_valid = (nb_teams & (nb_teams - 1)) == 0;

    let nb_rounds = (*nb_teams as f32).log2() as u32;
    let mut nb_match = nb_teams / 2;

    if !is_nb_valid {
        html!(
            <div>{"Number of teams is invalid. Must be a power of 2"}</div>
        )
    } else {
        html! {
            <div class={"bracket"}>
                {(0..nb_rounds).map(|i| {
                    let nb_match_round = nb_match;
                    nb_match /= 2;

                    html!(
                        <BracketRound round_id={i} nb_match={nb_match_round} />
                    )
                }).collect::<Html>()}
                <div class="round">
                    <div class={"ml-4 p-1 font-bold text-center bg-nutLighter"}>{"Winner"}</div>
                </div>
            </div>
        }
    }
}