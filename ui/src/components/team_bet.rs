use yew::prelude::*;

use crate::utils::utils::team_color_wrapper;

#[derive(Clone, PartialEq, Debug)]
pub struct Bet {
    pub name: String,
    pub bet_value: i32,
}

#[derive(PartialEq, Properties)]
pub struct TeamBetProps {
    pub team_name: String,
    pub bets: Vec<Bet>
}

#[function_component]
pub fn TeamBet(props: &TeamBetProps) -> Html {
    let TeamBetProps { team_name, bets } = props;

    html! {
        <div class="flex flex-col drop-shadow-lg">
            <div style={team_color_wrapper(team_name.to_string())} class="team-bg-color p-4 text-3xl text-white rounded-t">{team_name}</div>
            <ul class="bg-white max-h-[15rem] overflow-y-scroll rounded-b p-2">
                {
                    bets.iter().map(|bet| {
                        html!{<li>
                            {format!("{} - {}", bet.name, bet.bet_value)}
                        </li>}
                    }).collect::<Html>()
                }
            </ul>
        </div>
    }
}