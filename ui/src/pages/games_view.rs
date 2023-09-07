use crate::{
    components::{
        backlink::Backlink,
        bracket::Match,
        button::Button,
        form_input::FormInput,
        team_bet::{Bet, TeamBet}, loading_circle::LoadingCircle,
    },
    layouts::homelayout::HomeLayout,
    routers::Route,
    utils::utils::team_color_wrapper, api::{models::GameWithTeams, self},
};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, HtmlInputElement};
use yew::prelude::*;
use yew_hooks::use_effect_once;

#[derive(PartialEq, Properties)]
pub struct MatchViewProps {
    pub match_id: i32,
    pub tournament_id: i32,
}

#[function_component]
pub fn MatchView(props: &MatchViewProps) -> Html {
    let MatchViewProps { match_id, tournament_id } = props;


    let game: UseStateHandle<Option<GameWithTeams>> = use_state(|| None);
    let loading = use_state(|| true);

    {
        let game = game.clone();
        let loading = loading.clone();
        let match_id: i32 = match_id.clone();

        use_effect_once(move || {
            spawn_local(async move {
                game.set(api::game::get(match_id).await.ok());
                loading.set(false);
            });

            || ()
        });
    }

    let user_nut = use_state(|| 42);
    let user_has_win = use_state(|| true);
    let bet_made: UseStateHandle<Option<Bet>> = use_state(|| None);

    // BET 1
    let bets_1 = use_state(|| {
        let mut bets = vec![
            Bet {
                name: "SasuKey".to_string(),
                bet_value: 48,
            },
            Bet {
                name: "stepBr0".to_string(),
                bet_value: 71,
            },
            Bet {
                name: "Alain Terrieur".to_string(),
                bet_value: 1337,
            },
        ];
        bets.sort_by(|a, b| b.bet_value.cmp(&a.bet_value));
        bets
    });
    let total_1 = use_state(|| 0);
    {
        let bets_1_clone = bets_1.clone();
        let total_1 = total_1.clone();
        use_effect_with_deps(
            move |_| {
                total_1.set(bets_1_clone.iter().fold(0, |acc, bet| acc + bet.bet_value));
            },
            bets_1.clone(),
        );
    }

    let on_bet_click = |is_team_left: bool| {
        let bet_made = bet_made.clone();
        let game = game.clone();

        Callback::from(move |_| {
            if let Some(game) = (*game).clone() {
                let window = window().unwrap();
                let document = window.document().unwrap();
                let input_element = document.get_element_by_id("nut_bet").unwrap();
                let input_element = input_element.dyn_into::<HtmlInputElement>().ok();
                if let Some(input_element) = input_element {
                    if let Ok(bet_value) = input_element.value().parse() {
                        if bet_value > 0 {
                            bet_made.set(Some(Bet {
                                name: if is_team_left { game.team2.name.clone() } else { game.team1.name.clone() },
                                bet_value,
                            }));
                        }
                    }
                }
            }
        })
    };

    // BET 2
    let bets_2 = use_state(|| {
        let mut bets = vec![
            Bet {
                name: "JackJack".to_string(),
                bet_value: 41,
            },
            Bet {
                name: "XX_0n1_CHAN_XX".to_string(),
                bet_value: 9,
            },
            Bet {
                name: "IDIOT_DU_V1LL4G3".to_string(),
                bet_value: 3201,
            },
            Bet {
                name: "Jean-Michel".to_string(),
                bet_value: 1,
            },
            Bet {
                name: "Jean-Paul".to_string(),
                bet_value: 2,
            },
            Bet {
                name: "Jean-Yves".to_string(),
                bet_value: 37,
            },
            Bet {
                name: "Jean-Neymar".to_string(),
                bet_value: 4,
            },
            Bet {
                name: "Jean-Peuplus".to_string(),
                bet_value: 57,
            },
            Bet {
                name: "Jean-Fou".to_string(),
                bet_value: 88,
            },
            Bet {
                name: "Jean-Pierre".to_string(),
                bet_value: 8,
            },
            Bet {
                name: "Jean-Sébastien".to_string(),
                bet_value: 67,
            },
            Bet {
                name: "Jean-Augustin".to_string(),
                bet_value: 4,
            },
            Bet {
                name: "Jean-Jean".to_string(),
                bet_value: 7,
            },
            Bet {
                name: "Jean-Jeannod".to_string(),
                bet_value: 5,
            },
        ];
        bets.sort_by(|a, b| b.bet_value.cmp(&a.bet_value));
        bets
    });
    let total_2 = use_state(|| 0);
    {
        let bets_2_clone = bets_2.clone();
        let total_2 = total_2.clone();
        use_effect_with_deps(
            move |_| {
                total_2.set(bets_2_clone.iter().fold(0, |acc, bet| acc + bet.bet_value));
            },
            bets_2.clone(),
        );
    }

    let ratio = use_state(|| "".to_string());
    {
        let ratio = ratio.clone();
        use_effect_with_deps(
            move |(total_1, total_2)| {
                let total_1 = (**total_1) as f64;
                let total_2 = (**total_2) as f64;
                let total = total_1 + total_2;
                if total_1 < total_2 {
                    ratio.set(format!("1 : {:.2}", total_2 / total_1));
                } else {
                    ratio.set(format!("{:.2} : 1", total_1 / total_2));
                }
            },
            (total_1.clone(), total_2.clone()),
        );
    }
    let on_revert_bet_click = {
        let bet_made = bet_made.clone();
        Callback::from(move |_| {
            bet_made.set(None);
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
                            <div class="flex gap-5">
                                <TeamBet total={(*total_1).clone()} is_left={true} team_name={game.team1.name.clone()} bets={(*bets_1).clone()}/>
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
                                        if let Some(bet_made) = (*bet_made).clone() {
                                            <div class="text-xl text-center my-[0.92rem]">{format!("Vous avez misé {} noix sur \"{}\"", bet_made.bet_value, bet_made.name)}</div>
                                            <div style={team_color_wrapper((*bet_made.name).to_string())} class="flex rounded team-bg-color">
                                                <Button class="bg-transparent px-4 py-3 w-full" onclick={on_revert_bet_click}>
                                                    {"Annuler la mise"}
                                                </Button>
                                            </div>
                                        } else {
                                            <FormInput id="nut_bet" label="Nombre de noix à miser" form_type="number" min_num={1} required={true}/>
                                            <div class="flex relative drop-shadow-lg">
                                                <div style={team_color_wrapper(game.team1.name.clone())} class="flex grow-[1] hover:duration-[200ms] duration-[600ms] hover:grow-[3] rounded-l team-bg-color">
                                                    <Button class="bg-transparent px-4 py-3 text-right w-full" onclick={on_bet_click(false)}>
                                                        {format!("Miser sur \"{}\"", game.team1.name.clone())}
                                                    </Button>
                                                </div>
                                                <div style={team_color_wrapper(game.team2.name.clone())} class="flex grow-[1] hover:duration-[200ms] duration-[600ms] hover:grow-[3] rounded-r team-bg-color">
                                                    <Button class="bg-transparent px-4 py-3 text-left w-full" onclick={on_bet_click(true)}>
                                                        {format!("Miser sur \"{}\"", game.team2.name.clone())}
                                                    </Button>
                                                </div>
                                            </div>
                                        }
                                    } else {
                                        if let Some(bet_made) = (*bet_made).clone() {
                                            <div class="mt-2 flex flex-col items-center">
                                                <h2>{if *user_has_win { "BRAVO !" } else {"DOMMAGE !"}}</h2>
                                                {format!("Vous avez parié {} noix sur ce match.", bet_made.bet_value.clone())}
                                                <div>{if *user_has_win { format!("Vous avez remporté {} noix !", 42) } else { format!("Vous avez perdu {} noix !", 42) }}</div>
                                            </div>
                                        } else {
                                            <div class="mt-2">
                                                {format!("Impossible de faire une mise sur ce match car il est {} !", if game.status == 1 { "en cours" } else {"terminé"})}
                                            </div>
                                        }
                                    }
                                </div>
                                <TeamBet total={(*total_2).clone()} is_left={false} team_name={game.team2.name.clone()} bets={(*bets_2).clone()}/>
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
