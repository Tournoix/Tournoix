use yew::prelude::*;
use crate::{layouts::homelayout::HomeLayout, routers::Route, components::{backlink::Backlink, form_input::FormInput, button::Button, team_bet::{Bet, TeamBet}}, utils::utils::team_color_wrapper};

#[derive(PartialEq, Properties)]
pub struct MatchViewProps {
    pub id: i32,
}

#[function_component]
pub fn MatchView(props: &MatchViewProps) -> Html {
    let MatchViewProps { id } = props;

    let user_nut = use_state(|| 42);

    let team_name_1 = use_state(|| "Cloud9".to_string());
    let bets_1 = use_state(|| {
        let mut bets = vec![
            Bet { name: "SasuKey".to_string(), bet_value: 48 },
            Bet { name: "stepBr0".to_string(), bet_value: 71 },
            Bet { name: "Alain Terrieur".to_string(), bet_value: 1337 },
        ];
        bets.sort_by(|a, b| b.bet_value.cmp(&a.bet_value));
        bets
    });

    let team_name_2 = use_state(|| "fnatic".to_string());
    let bets_2 = use_state(|| {
        let mut bets = vec![
            Bet { name: "JackJack".to_string(), bet_value: 41 },
            Bet { name: "XX_0n1_CHAN_XX".to_string(), bet_value: 9 },
            Bet { name: "IDIOT_DU_V1LL4G3".to_string(), bet_value: 3201 },
            Bet { name: "Jean-Michel".to_string(), bet_value: 1 },
            Bet { name: "Jean-Paul".to_string(), bet_value: 2 },
            Bet { name: "Jean-Yves".to_string(), bet_value: 37 },
            Bet { name: "Jean-Neymar".to_string(), bet_value: 4 },
            Bet { name: "Jean-Peuplus".to_string(), bet_value: 57 },
            Bet { name: "Jean-Fou".to_string(), bet_value: 88 },
            Bet { name: "Jean-Pierre".to_string(), bet_value: 8 },
            Bet { name: "Jean-Sébastien".to_string(), bet_value: 67 },
            Bet { name: "Jean-Augustin".to_string(), bet_value: 4 },
            Bet { name: "Jean-Jean".to_string(), bet_value: 7 },
            Bet { name: "Jean-Jeannod".to_string(), bet_value: 5 },
        ];
        bets.sort_by(|a, b| b.bet_value.cmp(&a.bet_value));
        bets
    });

    let on_bet_1_click = Callback::from(|_| {
        log::info!("Bet on team A");
    });

    let on_bet_2_click = Callback::from(|_| {
        log::info!("Bet on team B");
    });
    
    html! {
        <HomeLayout>
            <div class="h-full relative">
                <img src="/img/bullets_texture.svg" class="absolute opacity-[2%] sm:w-9/12 w-11/12 pointer-events-none left-[12.5%] max-h-full"/>
                <div class="relative font-bebas flex flex-col items-center h-full pb-16 pt-12 sm:w-9/12 w-11/12 mx-auto z-10">
                    <Backlink route={Route::TournoixView{ id: 42 }} label="Retour au tournoi"/>
                    <div class="flex">
                        <TeamBet team_name={(*team_name_1).clone()} bets={(*bets_1).clone()}/>
                        <form class="flex flex-col">
                            <img src="/img/versus_big.png" class="h-60"/>
                            <div class="text-xl text-center">{"1.44 : 0.69"}</div>
                            <div class="text-xl text-center">{format!("Vous avez {} noix", *user_nut)}</div>
                            <FormInput id="nut_bet" label="Nombre de noix à miser" form_type="number" min_num={1} required={true}/>
                            <div class="flex relative drop-shadow-lg">
                                <div style={team_color_wrapper((*team_name_1).clone())} class="flex grow-[1] hover:duration-[200ms] duration-[600ms] hover:grow-[3] rounded-l team-bg-color">
                                    <Button class="bg-transparent px-4 py-3 text-right w-full" onclick={on_bet_1_click}>
                                        {format!("Miser sur \"{}\"", (*team_name_1).clone())}
                                    </Button>
                                </div>
                                <div style={team_color_wrapper((*team_name_2).clone())} class="flex grow-[1] hover:duration-[200ms] duration-[600ms] hover:grow-[3] rounded-r team-bg-color">
                                    <Button class="bg-transparent px-4 py-3 text-left w-full" onclick={on_bet_2_click}>
                                        {format!("Miser sur \"{}\"", (*team_name_2).clone())}
                                    </Button>
                                </div>
                            </div>
                        </form>
                        <TeamBet team_name={(*team_name_2).clone()} bets={(*bets_2).clone()}/>
                    </div>
                </div>
            </div>
        </HomeLayout>
    }
}