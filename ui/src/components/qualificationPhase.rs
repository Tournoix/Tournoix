use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

use crate::{components::{bracket::Match, checkbox::CheckBox}, utils::utils::team_color_wrapper};

#[derive(PartialEq, Properties)]
pub struct QualificationPhaseProps {
    pub on_started_click: Option<Callback<i32>>,
    pub on_finished_click: Option<Callback<i32>>,
    pub on_score1_change: Option<Callback<(i32, i32)>>,
    pub on_score2_change: Option<Callback<(i32, i32)>>,
}

#[function_component]
pub fn QualificationPhase(props: &QualificationPhaseProps) -> Html {
    let QualificationPhaseProps { on_started_click, on_finished_click, on_score1_change, on_score2_change } = props;

    let group_matches = use_context::<UseStateHandle<Vec<Vec<Match>>>>().expect("Missing group_matches provider");

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
        <div class="w-full mt-4">
            <ul class="flex flex-wrap gap-3 justify-center items-center">
                {
                    group_matches.iter().enumerate().map(|(index, group_match)| {
                        html!{<li class="rounded relative basis-72 bg-nutLighter flex flex-col justify-center items-center">
                            <h3 class="text-center">{"Groupe "}{index + 1}</h3>
                            <ul>
                                {
                                    group_match.iter().enumerate().map(|(index_2, _match)| {
                                        html!{<div>
                                            <hr class="m-0 border-nutLight drop-shadow-none"/>
                                            <li class="rounded relative flex justify-center items-center">
                                                <div style={team_color_wrapper(_match.team1.clone())} class="team-border-color border-r-4 px-2 m-2 rounded-l bg-nutLight w-24 text-right">
                                                    {_match.team1.clone()}
                                                </div>
                                                <input type="number" value={_match.score1.to_string()} disabled={if let Some(on_score1_change) = on_score1_change { false } else { true }} onchange={(change_score1.clone())(_match.id.clone())} class="mr-1 w-8 h-5 bg-white text-center" />
                                                {" - "}
                                                <input type="number" value={_match.score2.to_string()} disabled={if let Some(on_score2_change) = on_score2_change { false } else { true }} onchange={(change_score2.clone())(_match.id.clone())} class="ml-1 w-8 h-5 bg-white text-center" />
                                                <div style={team_color_wrapper(_match.team2.clone())} class="team-border-color border-l-4 px-2 m-2 rounded-r bg-nutLight w-24">
                                                    {_match.team2.clone()}
                                                </div>
                                                <div class="flex flex-col mr-2 mb-1">
                                                    {
                                                        if _match.started && _match.finished {
                                                            html!{<div class="font-bebas w-full text-xs rounded m-1 text-center text-white bg-green-600">{"TERMINÉ"}</div>}
                                                        } else if _match.started {
                                                            html!{<div class="font-bebas w-full text-xs rounded m-1 text-center text-white bg-yellow-600">{"EN COURS"}</div>}
                                                        } else {
                                                            html!{<div class="font-bebas w-full text-xs rounded m-1 text-center text-white bg-orange-600">{"EN ATTENTE"}</div>}
                                                        }
                                                    }
                                                    if let Some(on_started_click) = on_started_click {
                                                        <CheckBox class="m-0 text-xs" id={format!("started_{}", _match.id.clone())} label="Started" checked={_match.started.clone()} on_click={on_click_started(_match.id)}/>
                                                    }
                                                    if let Some(on_finished_click) = on_finished_click {
                                                        <CheckBox class="m-0 text-xs" id={format!("finished_{}", _match.id.clone())} label="Finished" checked={_match.finished.clone()} on_click={on_click_finished(_match.id)}/>
                                                    }
                                                </div>
                                            </li>
                                        </div>}
                                    }).collect::<Html>()
                                }
                            </ul>
                        </li>}
                    }).collect::<Html>()
                }
            </ul>
        </div>
    }
}