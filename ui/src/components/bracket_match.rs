use yew::prelude::*;

use super::bracket::Match;

#[derive(PartialEq, Properties)]
pub struct BracketMatchProps {
    pub game: Match
}

#[function_component]
pub fn BracketMatch(props: &BracketMatchProps) -> Html {
    let BracketMatchProps {game} = props;

    html! {
        <>
            <li class={"game game-top"}>
                <div class={classes!("bg-nutLighter", "pl-2", if game.finished && game.score1 > game.score2 {"font-bold"} else {""})}>
                    {game.team1.clone()}
                    <span class={classes!(if game.finished {if game.score1 > game.score2 {"bg-green-300"} else {"bg-red-300"}} else {""})}>{game.score1}</span>
                </div>
            </li>
            <li class={"game game-spacer"}>{"\u{00a0}"}</li>
            <li class={"game game-bottom"}>
                <div class={classes!("bg-nutLighter", "pl-2", if game.finished && game.score2 > game.score1 {"font-bold"} else {""})}>
                    {game.team2.clone()}
                    <span class={classes!(if game.finished {if game.score2 > game.score1 {"bg-green-300"} else {"bg-red-300"}} else {""})}>{game.score2}</span>
                </div>
            </li>
        </>
    }
}