use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

use crate::components::checkbox::CheckBox;

use super::bracket::Match;

#[derive(PartialEq, Properties)]
pub struct BracketMatchProps {
    pub game: Match,
    pub on_started_click: Option<Callback<i32>>,
    pub on_finished_click: Option<Callback<i32>>,
    pub on_score1_change: Option<Callback<(i32, i32)>>,
    pub on_score2_change: Option<Callback<(i32, i32)>>,
}

#[function_component]
pub fn BracketMatch(props: &BracketMatchProps) -> Html {
    let BracketMatchProps {game, on_started_click, on_finished_click, on_score1_change, on_score2_change} = props;

    let on_click_started = |id: i32| {
        if let Some(on_started_click) = on_started_click {
            let on_started_click = on_started_click.clone();

            Callback::from(move |_| {
                on_started_click.emit(id);
                ()
            })
        } else {
            Callback::noop()
        }
    };

    let on_click_finished = |id: i32| {
        if let Some(on_finished_click) = on_finished_click {
            let on_finished_click = on_finished_click.clone();

            Callback::from(move |_| {
                on_finished_click.emit(id);
                ()
            })
        } else {
            Callback::noop()
        }
    };

    let change_score1 = |id| {
        let on_score1_change = on_score1_change.clone();
        Callback::from(move |e: Event| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");

            // You must KNOW target is a HtmlInputElement, otherwise
            // the call to value would be Undefined Behaviour (UB).
            // Here we are sure that this is input element so we can convert it to the appropriate type without checking
            if let Ok(val) = target.unchecked_into::<HtmlInputElement>().value().parse::<i32>() {
                if let Some(on_score1_change) = &on_score1_change {
                    on_score1_change.emit((id, val));
                }
            }
        })
    };
    let change_score2 = |id| {
        let on_score2_change = on_score2_change.clone();
        Callback::from(move |e: Event| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");

            // You must KNOW target is a HtmlInputElement, otherwise
            // the call to value would be Undefined Behaviour (UB).
            // Here we are sure that this is input element so we can convert it to the appropriate type without checking
            if let Ok(val) = target.unchecked_into::<HtmlInputElement>().value().parse::<i32>() {
                if let Some(on_score2_change) = &on_score2_change {
                    on_score2_change.emit((id, val));
                }
            }
        })
    };

    html! {
        <>
            <li class={"game game-top"}>
                <div class={classes!("flex", "bg-nutLighter", "pl-2", if game.finished && game.score1 > game.score2 {"font-bold"} else {""})}>
                    {game.team1.clone()}
                    <input type="number" disabled={if let Some(on_score1_change) = on_score1_change { false } else { true }} onchange={change_score1(game.id)} value={game.score1.to_string()} class={classes!("game-input-score", if game.finished {if game.score2 < game.score1 {"bg-green-300"} else {"bg-red-300"}} else {""})}/>
                </div>
            </li>
            <li class={"game game-spacer"}>
                {
                    if game.started && game.finished {
                        html!{<div class="font-bebas ml-auto px-3 text-xs rounded m-1 text-center text-white bg-green-600">{"TERMINÉ"}</div>}
                    } else if game.started {
                        html!{<div class="font-bebas ml-auto px-3 text-xs rounded m-1 text-center text-white bg-yellow-600">{"EN COURS"}</div>}
                    } else {
                        html!{<div class="font-bebas ml-auto px-3 text-xs rounded m-1 text-center text-white bg-orange-600">{"EN ATTENTE"}</div>}
                    }
                }
                if let Some(on_started_click) = on_started_click {
                    <CheckBox class="ml-auto text-xs" id={format!("elim_started_{}", game.id.clone())} label="Started" checked={game.started.clone()} on_click={on_click_started(game.id)}/>
                }
                if let Some(on_finished_click) = on_finished_click {
                    <CheckBox class="ml-auto text-xs" id={format!("elim_finished_{}", game.id.clone())} label="Finished" checked={game.finished.clone()} on_click={on_click_finished(game.id)}/>
                }
            </li>
            <li class={"game game-bottom"}>
                <div class={classes!("flex", "bg-nutLighter", "pl-2", if game.finished && game.score2 > game.score1 {"font-bold"} else {""})}>
                    {game.team2.clone()}
                    <input type="number" disabled={if let Some(on_score2_change) = on_score2_change { false } else { true }} onchange={change_score2(game.id)} value={game.score2.to_string()} class={classes!("game-input-score", if game.finished {if game.score2 > game.score1 {"bg-green-300"} else {"bg-red-300"}} else {""})}/>
                </div>
            </li>
        </>
    }
}