use crate::{
    components::{
        backlink::Backlink,
        button::Button,
        form_input::FormInput,
        team_bet::{TeamBet, BetItem}, loading_circle::LoadingCircle, user_provider::UserContext,
    },
    layouts::homelayout::HomeLayout,
    routers::Route,
    utils::utils::team_color_wrapper, api::{models::{GameWithTeams, self, BetWithUser}, self, game::BetData}, notification::{NotifType, CustomNotification},
};
use time::Duration;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, HtmlInputElement};
use yew::prelude::*;
use yew_hooks::{use_effect_once, use_interval};
use yew_notifications::use_notification;

#[derive(PartialEq, Properties)]
pub struct MatchViewProps {
    pub match_id: i32,
    pub tournament_id: i32,
}

#[function_component]
pub fn MatchView(props: &MatchViewProps) -> Html {
    let MatchViewProps { match_id, tournament_id } = props;

    let notifications_manager = use_notification::<CustomNotification>();

    let user_info = use_context::<UserContext>().expect("Missing user context provider");
    let game: UseStateHandle<Option<GameWithTeams>> = use_state(|| None);
    let user_bet: UseStateHandle<Option<models::Bet>> = use_state(|| None);
    let loading = use_state(|| true);
    let trigger = use_state(|| true);
    let bets_1: UseStateHandle<Vec<BetItem>> = use_state(|| vec![]);
    let bets_2: UseStateHandle<Vec<BetItem>> = use_state(|| vec![]);
    let total_1 = use_state(|| 0);
    let total_2 = use_state(|| 0);
    let ratio = use_state(|| "".to_string());
    let user_nut = use_state(|| 0);
    let user_gains = use_state(|| 0);

    {
        let game_clone = game.clone();
        let loading = loading.clone();
        let match_id: i32 = match_id.clone();
        let bets_1 = bets_1.clone();
        let bets_2 = bets_2.clone();
        let user_info = user_info.clone();
        let user_bet = user_bet.clone();
        let ratio = ratio.clone();
        let total_1 = total_1.clone();
        let total_2 = total_2.clone();
        let user_gains = user_gains.clone();
        let tournament_id = tournament_id.clone();
        let user_nut = user_nut.clone();

        use_interval(move || {
            let game = game_clone.clone();
            spawn_local(async move {
                let response = api::game::get(match_id).await.ok();
                if let Some(response) = response {
                    if let Some(game_inner) = &*game {
                        if response.status != game_inner.status || response.score1 != game_inner.score1 || response.score2 != game_inner.score2 {
                            game.set(Some(response));
                        }
                    }
                }
            });

            let game_clone_2 = game_clone.clone();
            let ratio = ratio.clone();
            let total_1 = total_1.clone();
            let total_2 = total_2.clone();
            let total_1_clone = total_1.clone();
            let total_2_clone = total_2.clone();
            let bets_1 = bets_1.clone();
            let bets_2 = bets_2.clone();
            let user_info = user_info.clone();
            let user = user_info.user.clone();
            let tournament_id = tournament_id.clone();
            let user_nut = user_nut.clone();

            // fetch bets
            spawn_local(async move {
                let response = api::bets::get_bets(match_id.clone() as i32).await;
                
                if let Ok(response) = response {
                    if let Some(game) = (*game_clone_2).clone() {
                        if let Some(user) = user {
                            if let Some(nut) = api::game::get_nb_nut(tournament_id.clone()).await.ok() {
                                if (nut.stock) != *user_nut {
                                    user_nut.set(nut.stock);
                                }
                            }

                            let mut _bets_1: Vec<BetItem> = vec![];
                            let mut _bets_2: Vec<BetItem> = vec![];
    
                            // sort response from nb_nut
                            let mut response: Vec<BetWithUser> = response.clone();
                            response.sort_by(|a, b| b.nb_nut.cmp(&a.nb_nut));
        
                            // fill bets_1 and bets_2
                            for bet in response.iter() {
        
                                let mut _bet = (*bet).clone();
                                
                                if _bet.fk_teams == game.team1.id {
                                    _bets_1.push(BetItem {
                                        name: _bet.username.clone(),
                                        nb_nut: _bet.nb_nut.clone(),
                                    });
                                } else {
                                    _bets_2.push(BetItem {
                                        name: _bet.username.clone(),
                                        nb_nut: _bet.nb_nut.clone(),
                                    });
                                }
                            }

                            // total_1 et total_2
                            let total_1_val = _bets_1.iter().fold(0, |acc, bet| acc + bet.nb_nut);
                            let total_2_val = _bets_2.iter().fold(0, |acc, bet| acc + bet.nb_nut);
                            if total_1_val != *total_1 { total_1.set(total_1_val); }
                            if total_2_val != *total_2 { total_2.set(total_2_val); }

                            // ratio
                            let total_1_val = total_1_val as f64;
                            let total_2_val = total_2_val as f64;
                            let total = total_1_val + total_2_val;
                            let ratio_val: String;
                            if total_1_val < total_2_val {
                                ratio_val = if total_1_val == 0. { "1 : 1".to_string() } else { format!("1 : {:.2}", total_2_val / total_1_val) };
                            } else {
                                ratio_val = if total_2_val == 0. { "1 : 1".to_string() } else { format!("{:.2} : 1", total_1_val / total_2_val) };
                            }
                            if ratio_val != *ratio {
                                ratio.set(ratio_val);
                            }

                            // Only set bets_1 and bets_2 if values have changed
                            let mut same = true;
                            for bet in _bets_1.iter() {
                                for _bet in bets_1.iter() {
                                    if bet.name != _bet.name || bet.nb_nut != _bet.nb_nut {
                                        same = false;
                                    }
                                }
                            }
                            if !same || bets_1.len() != _bets_1.len() {
                                bets_1.set(_bets_1);
                            }

                            same = true;

                            for bet in _bets_2.iter() {
                                for _bet in bets_2.iter() {
                                    if bet.name != _bet.name || bet.nb_nut != _bet.nb_nut {
                                        same = false;
                                    }
                                }
                            }
                            if !same || bets_2.len() != _bets_2.len() {
                                bets_2.set(_bets_2);
                            }
                        }
                    }
                }
            });

            let game_clone_3 = game_clone.clone();
            let user_gains = user_gains.clone();
            let user_bet = user_bet.clone();

            // Check if user has win
            spawn_local(async move {
                if let Some(game) = (*game_clone_3).clone() {
                    if let Some(user_bet) = (*user_bet).clone() {
                        let has_win = game.score1 > game.score2 && user_bet.fk_teams == game.team1.id || game.score1 < game.score2 && user_bet.fk_teams == game.team2.id;

                        if has_win {
                            if game.score1 > game.score2 {
                                if (*total_1_clone) != 0 {
                                    let participation_percentage = user_bet.nb_nut as f64 / (*total_1_clone) as f64;
                                    let gains = participation_percentage * (*total_2_clone) as f64;
                                    user_gains.set(gains as i32);
                                } else {
                                    user_gains.set(0);
                                }
                            } else {
                                if (*total_2_clone) != 0 {
                                    let participation_percentage = user_bet.nb_nut as f64 / (*total_2_clone) as f64;
                                    let gains = participation_percentage * (*total_1_clone) as f64;
                                    user_gains.set(gains as i32);
                                } else {
                                    user_gains.set(0);
                                }
                            };
                        } else {
                            user_gains.set(-user_bet.nb_nut);
                        }
                    }
                }
            });
        }, 1000);

        let game = game.clone();

        use_effect_once(move || {
            spawn_local(async move {
                game.set(api::game::get(match_id).await.ok());
                loading.set(false);
            });

            || ()
        });
    }

    {
        let user_bet = user_bet.clone();
        let user_nut = user_nut.clone();
        let user_info = user_info.clone();
        let user = user_info.user.clone();
        let trigger = trigger.clone();
        let match_id = match_id.clone();
        let tournament_id = tournament_id.clone();
        let game = game.clone();
        let loading = loading.clone();

        use_effect_with_deps(
            move |_| {
                if let Some(user) = user {
                    {
                        let user = user.clone();
                        let user_bet = user_bet.clone();
                        let user_nut = user_nut.clone();
                        let match_id = match_id.clone();
                        let tournament_id = tournament_id.clone();
                        let game = game.clone();
                        let loading = loading.clone();

                        spawn_local(async move {
                            user_bet.set(api::game::get_user_bet_on_match(user.id.clone() as i32, match_id).await.ok());

                            if let Some(nut) = api::game::get_nb_nut(tournament_id.clone()).await.ok() {
                                user_nut.set(nut.stock);
                            }
                        });
                    }
                }

                || ()
            },
            (user_info, trigger.clone()),
        );
    }

    let local_team_name_from_id = |id| {
        if let Some(game) = (*game).clone() {
            if game.team1.id == id {
                return game.team1.name.clone();
            } else if game.team2.id == id {
                return game.team2.name.clone();
            } else {
                return "error".to_string();
            }
        }
        "error".to_string()
    };

    let on_bet_click = |team_id: i32| {
        let user_info = user_info.clone();
        let user = user_info.user.clone();
        let user_bet = user_bet.clone();
        let game = game.clone();
        let notifications_manager = notifications_manager.clone();
        let trigger = trigger.clone();
        let match_id = match_id.clone();

        Callback::from(move |_| {
            if let Some(game) = (*game).clone() {
                let window = window().unwrap();
                let document = window.document().unwrap();
                let input_element = document.get_element_by_id("nut_bet").unwrap();
                let input_element = input_element.dyn_into::<HtmlInputElement>().ok();
                if let Some(input_element) = input_element {
                    if let Ok(nb_nut) = input_element.value().parse::<i32>() {
                        if nb_nut > 0 {

                            let user_bet = user_bet.clone();
                            let notifications_manager = notifications_manager.clone();
                            let user = user.clone();
                            let trigger = trigger.clone();
                            let match_id = match_id.clone();

                            spawn_local(async move {
                                if let Some(user) = user {
                                    let res = api::game::bet(match_id.clone() as i32, BetData {
                                        team_id: team_id.clone(),
                                        nut: nb_nut.clone(),
                                    }).await;
                                    
                                    if let Ok(res) = res {
                                        user_bet.set(Some(res));

                                        /*notifications_manager.spawn(CustomNotification::new(
                                            "Vous avez misé",
                                            format!("Vous vous avez misé {} sur ce match.", nb_nut),
                                            NotifType::Success,
                                            Duration::seconds(5),
                                        ));*/
                                    } else {
                                        /*notifications_manager.spawn(CustomNotification::new(
                                            "Erreur",
                                            format!("Impossible de miser {} noix sur ce match.", nb_nut),
                                            NotifType::Error,
                                            Duration::seconds(5),
                                        ));*/
                                    }
                                }
                                
                                trigger.set(!*trigger);
                            });

                        }
                    }
                }
            }
        })
    };

    let on_revert_bet_click = {
        let user_info = user_info.clone();
        let user = user_info.user.clone();
        let user_bet = user_bet.clone();
        let notifications_manager = notifications_manager.clone();
        let trigger = trigger.clone();
        let match_id = match_id.clone();

        Callback::from(move |_| {
            if let Some(user) = user.clone() {
                let user_bet = user_bet.clone();
                let notifications_manager = notifications_manager.clone();
                let trigger = trigger.clone();
                let match_id = match_id.clone();

                spawn_local(async move {
                    if let Some(user_bet) = &*user_bet {
                        /*notifications_manager.spawn(CustomNotification::new(
                            "Mise annulée",
                            format!("Vous avez annulé votre mise de {}.", user_bet.nb_nut.clone()),
                            NotifType::Success,
                            Duration::seconds(5),
                        ));*/
                    }

                    user_bet.set(api::game::delete_bet(match_id.clone() as i32).await.ok());
                    
                    trigger.set(!*trigger);
                });
            }
        })
    };

    html! {
        <HomeLayout>
            <div class="h-full relative">
                <img src="/img/bullets_texture.svg" class="absolute opacity-[2%] sm:w-9/12 w-11/12 pointer-events-none left-[12.5%] max-h-full"/>
                <div class="relative font-bebas flex flex-col items-center h-full pb-16 pt-12 sm:w-9/12 w-11/12 mx-auto z-10">
                    <Backlink route={Route::TournoixView{ id: tournament_id.clone() }} label="Retour au tournoi"/>
                    if *loading {
                        <LoadingCircle />
                    } else {
                        if let Some(game) = &*game {
                            <div class="flex sm:flex-row flex-col gap-5">
                                <TeamBet total={(*total_1).clone()} is_left={true} team_name={game.team1.name.clone()} score={game.score1.clone()} bets={(*bets_1).clone()}/>
                                <div class="flex flex-col w-96">
                                    <img src="/img/versus_big.png" class="w-72 mx-auto"/>
                                    <div class="flex justify-center items-center mb-2">
                                        <span class="mr-1">{"Ce match est "}</span>
                                        if game.status == 2 {
                                            <div class="font-bebas px-2 py-1 text-xs rounded m-1 text-center text-white bg-green-600">{"TERMINÉ"}</div>
                                        } else if game.status == 1 {
                                            <div class="font-bebas px-2 py-1 text-xs rounded m-1 text-center text-white bg-yellow-600">{"EN COURS"}</div>
                                        } else {
                                            <div class="font-bebas px-2 py-1 text-xs rounded m-1 text-center text-white bg-orange-600">{"EN ATTENTE"}</div>
                                        }
                                    </div>
                                    <div class="text-xl text-center">{(*ratio).clone()}</div>
                                    <div class="text-xl text-center">{format!("Vous avez {} noix", *user_nut)}</div>
                                    if game.status == 0 {
                                        if let Some(user_bet) = &*user_bet {
                                            <div class="text-xl text-center my-[0.92rem]">{format!("Vous avez misé {} noix sur \"{}\"", user_bet.nb_nut, local_team_name_from_id(user_bet.fk_teams.clone()))}</div>
                                            <div style={team_color_wrapper(local_team_name_from_id(user_bet.fk_teams.clone()).to_string())} class="flex rounded team-bg-color">
                                                <Button class="bg-transparent px-4 py-3 w-full" onclick={on_revert_bet_click}>
                                                    {"Annuler la mise"}
                                                </Button>
                                            </div>
                                        } else {
                                            <FormInput id="nut_bet" label="Nombre de noix à miser" form_type="number" min_num={1} required={true}/>
                                            <div class="flex relative drop-shadow-lg">
                                                <div style={team_color_wrapper(game.team1.name.clone())} class="flex grow-[1] hover:duration-[200ms] duration-[600ms] hover:grow-[3] rounded-l team-bg-color">
                                                    <Button class="bg-transparent px-4 py-3 text-right w-full hover:tracking-normal" onclick={on_bet_click(game.team1.id.clone())}>
                                                        {format!("Miser sur \"{}\"", game.team1.name.clone())}
                                                    </Button>
                                                </div>
                                                <div style={team_color_wrapper(game.team2.name.clone())} class="flex grow-[1] hover:duration-[200ms] duration-[600ms] hover:grow-[3] rounded-r team-bg-color">
                                                    <Button class="bg-transparent px-4 py-3 text-left w-full hover:tracking-normal" onclick={on_bet_click(game.team2.id.clone())}>
                                                        {format!("Miser sur \"{}\"", game.team2.name.clone())}
                                                    </Button>
                                                </div>
                                            </div>
                                        }
                                    } else if game.status == 1 {
                                        if let Some(user_bet) = &*user_bet {
                                            <div class="text-xl text-center my-[0.92rem]">{format!("Vous avez misé {} noix sur \"{}\"", user_bet.nb_nut, local_team_name_from_id(user_bet.fk_teams.clone()))}</div>
                                        }
                                        <div class="mt-2">
                                            {format!("Impossible de faire une mise sur ce match car il est {} !", if game.status == 1 { "en cours" } else {"terminé"})}
                                        </div>
                                    } else if game.status == 2 {
                                        if let Some(user_bet) = &*user_bet {
                                            if *user_gains == 0 {
                                                <div class="mt-2 flex flex-col items-center">
                                                    {format!("Vous avez parié {} noix sur ce match.", user_bet.nb_nut.clone())}
                                                    <div>{"Vous n'avez rien gagné ni perdu."}</div>
                                                </div>
                                            } else {
                                                <div class="mt-2 flex flex-col items-center">
                                                    if *user_gains > 0 {
                                                        <h1 class="text-green-700">{"BRAVO !"}</h1>
                                                    } else {
                                                        <h1 class="text-red-700">{"DOMMAGE !"}</h1>
                                                    }
                                                    {format!("Vous avez parié {} noix sur ce match.", user_bet.nb_nut.clone())}
                                                    <div>{if *user_gains > 0 { format!("Vous avez remporté {} noix !", *user_gains) } else { format!("Vous avez perdu {} noix !", -*user_gains) }}</div>
                                                </div>
                                            }
                                        } else {
                                            <div class="mt-2">
                                                {format!("Impossible de faire une mise sur ce match car il est {} !", if game.status == 1 { "en cours" } else {"terminé"})}
                                            </div>
                                        }
                                    }
                                </div>
                                <TeamBet total={(*total_2).clone()} is_left={false} team_name={game.team2.name.clone()} score={game.score2.clone()} bets={(*bets_2).clone()}/>
                            </div>
                        } else  {
                            <div>{"Oups, ce match n'existe pas :("}</div>
                        }
                    }
                </div>
            </div>
        </HomeLayout>
    }
}
