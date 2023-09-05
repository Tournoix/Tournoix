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
    pub bets: Vec<Bet>,
    pub total: i32, // Computed in the component above
    pub is_left: bool,
}

#[function_component]
pub fn TeamBet(props: &TeamBetProps) -> Html {
    let TeamBetProps { team_name, bets, total, is_left } = props;

    html! {
        <div class="flex flex-col drop-shadow-lg">
            <div style={team_color_wrapper(team_name.to_string())} class="team-bg-color p-4 text-3xl text-white rounded-t">{team_name}</div>
            <ul class={format!("bg-white max-h-[17rem] overflow-y-scroll {}", if is_left.clone() { "text-right" } else { "" })}>
                {
                    bets.iter().enumerate().map(|(index, bet)| {
                        html!{<li class={if index % 2 == 0 { "px-2 py-1 " } else { "px-2 py-1 bg-[#f1f1f1]" }}>
                            if is_left.clone() {
                                {format!("{} - {}", bet.name, bet.bet_value)}
                            } else {
                                {format!("{} - {}", bet.bet_value, bet.name)}
                            }
                        </li>}
                    }).collect::<Html>()
                }
            </ul>
            <div class={format!("bg-white p-2 rounded-b border-t {}", if is_left.clone() { "text-right" } else { "" })}>
                {format!("Total: {} noix", total.to_string())}
            </div>
        </div>
    }
}