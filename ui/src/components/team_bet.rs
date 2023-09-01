use yew::prelude::*;

use crate::components::{button::Button, form_input::FormInput};

#[derive(PartialEq, Properties)]
pub struct TeamBetProps {}

#[function_component]
pub fn TeamBet(props: &TeamBetProps) -> Html {
    let TeamBetProps {} = props;

    let on_bet_left_click = Callback::from(|_| {
        log::info!("Bet on team A");
    });

    let on_bet_right_click = Callback::from(|_| {
        log::info!("Bet on team B");
    });

    html! {
        <div class="bg-purple-200">
            {"TeamBet component"}
            <div class="flex">
                <div class="bg-yellow-200 mx-5 my-16 p-4 text-3xl text-green-800">{"EQUIPE A"}</div>
                <form class="flex flex-col">
                    <img src="/img/versus.png"/>
                    <div class="text-xl text-center">{"1.44 : 0.69"}</div>
                    <div class="text-xl text-center">{format!("Vous avez {} noix", 42)}</div>
                    <FormInput id="nut_bet" label="Nombre de noix à miser" form_type="number" min_num={1} required={true}/>
                    <div class="flex gap-4">
                        <Button class="p-4" onclick={on_bet_left_click}>
                            {"Miser sur l'équipe A"}
                        </Button>
                        <Button class="p-4" onclick={on_bet_right_click}>
                            {"Miser sur l'équipe B"}
                        </Button>
                    </div>
                </form>
                <div class="bg-yellow-200 mx-5 my-16 p-4 text-3xl text-orange-800">{"EQUIPE B"}</div>
            </div>
        </div>
    }
}