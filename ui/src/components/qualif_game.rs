use time::Duration;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yew_notifications::use_notification;

use crate::{
    api::{
        self,
        models::{GameUpdate, GameWithTeams},
    },
    notification::{CustomNotification, NotifType},
    utils::utils::team_color_wrapper,
};

#[derive(PartialEq, Properties)]
pub struct QualifGameProps {
    pub game: GameWithTeams,
    #[prop_or_default]
    pub editable: bool,
    #[prop_or_default]
    pub on_game_update: Callback<i32>,
}

#[function_component]
pub fn QualifGame(props: &QualifGameProps) -> Html {
    let QualifGameProps {
        game,
        editable,
        on_game_update,
    } = props;

    let notifs = use_notification::<CustomNotification>();

    let on_click_start = {
        let game = game.clone();
        let notifs = notifs.clone();
        let on_game_update = on_game_update.clone();

        Callback::from(move |_| {
            let game = game.clone();
            let notifs = notifs.clone();
            let on_game_update = on_game_update.clone();

            spawn_local(async move {
                match api::games::update(
                    game.id,
                    GameUpdate {
                        score1: None,
                        score2: None,
                        phase: None,
                        status: Some(1),
                    },
                )
                .await
                {
                    Ok(game) => {
                        on_game_update.emit(game.id);
                    }

                    Err(e) => {
                        notifs.spawn(CustomNotification::new(
                            format!("Erreur: {}", e.error.reason),
                            e.error.description,
                            NotifType::Error,
                            Duration::seconds(5),
                        ));
                    }
                }
            });
        })
    };

    let on_click_finish = {
        let game_id = game.id;
        let notifs = notifs.clone();
        let on_game_update = on_game_update.clone();

        Callback::from(move |_| {
            let notifs = notifs.clone();
            let on_game_update = on_game_update.clone();

            spawn_local(async move {
                match api::games::close(
                    game_id
                )
                .await
                {
                    Ok(_) => {
                        on_game_update.emit(game_id);
                    }

                    Err(e) => {
                        notifs.spawn(CustomNotification::new(
                            format!("Erreur: {}", e.error.reason),
                            e.error.description,
                            NotifType::Error,
                            Duration::seconds(5),
                        ));
                    }
                }
            });
        })
    };

    let on_click_cancel = {
        let game_id = game.id;
        let notifs = notifs.clone();
        let on_game_update = on_game_update.clone();

        Callback::from(move |_| {
            let notifs = notifs.clone();
            let on_game_update = on_game_update.clone();

            spawn_local(async move {
                match api::games::update(
                    game_id,
                    GameUpdate {
                        score1: None,
                        score2: None,
                        phase: None,
                        status: Some(0),
                    },
                )
                .await
                {
                    Ok(game) => {
                        on_game_update.emit(game.id);
                    }

                    Err(e) => {
                        notifs.spawn(CustomNotification::new(
                            format!("Erreur: {}", e.error.reason),
                            e.error.description,
                            NotifType::Error,
                            Duration::seconds(5),
                        ));
                    }
                }
            });
        })
    };

    let change_score1 = {
        let game_id = game.id;
        let notifs = notifs.clone();

        Callback::from(move |e: Event| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");

            // You must KNOW target is a HtmlInputElement, otherwise
            // the call to value would be Undefined Behaviour (UB).
            // Here we are sure that this is input element so we can convert it to the appropriate type without checking
            if let Ok(val) = target
                .unchecked_into::<HtmlInputElement>()
                .value()
                .parse::<i32>()
            {
                let notifs = notifs.clone();

                spawn_local(async move {
                    match api::games::update(
                        game_id,
                        GameUpdate {
                            score1: Some(val),
                            score2: None,
                            phase: None,
                            status: None,
                        },
                    )
                    .await
                    {
                        Ok(_) => {},
    
                        Err(e) => {
                            notifs.spawn(CustomNotification::new(
                                format!("Erreur: {}", e.error.reason),
                                e.error.description,
                                NotifType::Error,
                                Duration::seconds(5),
                            ));
                        }
                    }
                });
            }
        })
    };

    let change_score2 = {
        let game_id = game.id;
        let notifs = notifs.clone();

        Callback::from(move |e: Event| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");

            // You must KNOW target is a HtmlInputElement, otherwise
            // the call to value would be Undefined Behaviour (UB).
            // Here we are sure that this is input element so we can convert it to the appropriate type without checking
            if let Ok(val) = target
                .unchecked_into::<HtmlInputElement>()
                .value()
                .parse::<i32>()
            {
                let notifs = notifs.clone();

                spawn_local(async move {
                    match api::games::update(
                        game_id,
                        GameUpdate {
                            score1: None,
                            score2: Some(val),
                            phase: None,
                            status: None,
                        },
                    )
                    .await
                    {
                        Ok(_) => {},
    
                        Err(e) => {
                            notifs.spawn(CustomNotification::new(
                                format!("Erreur: {}", e.error.reason),
                                e.error.description,
                                NotifType::Error,
                                Duration::seconds(5),
                            ));
                        }
                    }
                });
            }
        })
    };

    html! {
        <div>
            <hr class="m-0 border-nutLight drop-shadow-none"/>
            <li class="rounded relative flex justify-center items-center">
                <div style={team_color_wrapper(game.team1.name.clone())} class="team-border-color border-r-4 px-2 m-2 rounded-l bg-nutLight w-24 text-right">
                    {game.team1.name.clone()}
                </div>
                <input type="number" value={game.score1.to_string()} disabled={!editable || game.status != 1} onchange={change_score1} class="mr-1 w-8 h-5 bg-white text-center" />
                {" - "}
                <input type="number" value={game.score2.to_string()} disabled={!editable || game.status != 1} onchange={change_score2} class="ml-1 w-8 h-5 bg-white text-center" />
                <div style={team_color_wrapper(game.team2.name.clone())} class="team-border-color border-l-4 px-2 m-2 rounded-r bg-nutLight w-24">
                    {game.team2.name.clone()}
                </div>
                <div class="flex flex-col mr-2 mb-1">
                    {
                        if game.status == 2 {
                            html!{<div class="font-bebas w-full text-xs rounded m-1 text-center text-white bg-green-600">{"TERMINÉ"}</div>}
                        } else if game.status == 1 {
                            html!{<div class="font-bebas w-full text-xs rounded m-1 text-center text-white bg-yellow-600">{"EN COURS"}</div>}
                        } else {
                            html!{<div class="font-bebas w-full text-xs rounded m-1 text-center text-white bg-orange-600">{"EN ATTENTE"}</div>}
                        }
                    }
                    if *editable {
                        <div class="mt-1">
                            if game.status == 0 {
                                <button class="font-bebas text-sm rounded bg-green-500 px-1 w-full mx-1 text-white" onclick={on_click_start}>{"Démarrer"}</button>
                            }

                            if game.status > 0 {
                                <button class="font-bebas text-sm rounded bg-red-500 px-1 w-full mx-1 text-white" onclick={on_click_cancel}>{"Annuler"}</button>
                            }

                            if game.status == 1 {
                                <button class="font-bebas text-sm rounded bg-green-500 px-1 w-full mx-1 text-white" onclick={on_click_finish}>{"Terminer"}</button>
                            }
                        </div>
                    }
                </div>
            </li>
        </div>
    }
}
