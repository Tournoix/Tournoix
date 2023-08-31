use yew::prelude::*;

use crate::components::bracket_match::BracketMatch;

#[derive(PartialEq, Properties)]
pub struct BracketRoundProps {
    pub nb_match: usize,
    pub round_id: u32
}

#[function_component]
pub fn BracketRound(props: &BracketRoundProps) -> Html {
    let BracketRoundProps {nb_match, round_id} = props;
    let round_title = format!("Round {}", round_id+1);

    html! {
        <ul class={" round"}>
            {(0..nb_match.clone()).map(|i| {
                html!(
                    <>
                        <li class={"spacer"}>
                            {if i == 0 {
                                html!(
                                    <div class={"bg-nutLight text-center border"}>{&round_title}</div>
                                )
                            } else {
                                html!()
                            }}
                            {"\u{00a0}"}
                        </li>
                        <BracketMatch />
                    </>
                )
            }).collect::<Html>()}
            <li class={"spacer"}>{"\u{00a0}"}</li>
        </ul>
    }
}