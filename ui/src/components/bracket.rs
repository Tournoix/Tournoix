use yew::prelude::*;
use crate::components::bracket_round::BracketRound;

#[derive(Clone, PartialEq, Debug)]
pub struct Match {
    pub id: i32,
    pub team1: String,
    pub score1: i32,
    pub team2: String,
    pub score2: i32,
    pub started: bool,
    pub finished: bool
}

pub type BracketTeams = Vec<Vec<Match>>;

#[derive(PartialEq, Properties)]
pub struct BracketProps {
    pub teams: BracketTeams
}

#[function_component]
pub fn Bracket(props: &BracketProps) -> Html {
    let BracketProps {teams} = props;

    let is_nb_valid = teams.len() > 0 && (teams[0].len() & (teams[0].len() - 1)) == 0;

    if !is_nb_valid {
        html!(
            <div>{"Number of teams is invalid. Must be a power of 2"}</div>
        )
    } else {
        let nb_rounds = (teams.get(0).unwrap().len() as f32).log2() as u32;
        let mut nb_match = teams.get(0).unwrap().len() / 2;
        let mut round_id = 0;

        html! {
            <div class={"bracket"}>
                {teams.iter().map(|round| {
                    round_id += 1;
                    html!(
                        <BracketRound round_id={round_id} matches={round.clone()} />
                    )
                }).collect::<Html>()}
                <div class="round">
                    <div class={"ml-4 p-1 font-bold text-center bg-nutLighter"}>{"Winner"}</div>
                </div>
            </div>
        }
    }
}