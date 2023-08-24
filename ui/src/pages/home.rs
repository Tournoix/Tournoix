use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{routers::Route, layouts::homelayout::HomeLayout, components::button::Button};

#[derive(PartialEq, Properties)]
pub struct HomeProps {}

#[function_component]
pub fn Home(props: &HomeProps) -> Html {
    let HomeProps {} = props;
    let navigator = use_navigator().unwrap();
    
    let onclick = Callback::from(move |_| navigator.push(&Route::Tournoix));

    html! {
        <HomeLayout>
            <div class={"flex flex-col items-center font-bebas h-full z-10 relative"}>
                <div class="wavy absolute top-0 w-full h-full z-0 pointer-events-none opacity-30"></div>
                /*<img src="/img/bullets_texture.svg" class="absolute opacity-5 w-full"/>*/
                <img src="/img/hero_nut.png" class="h-32 mt-16 mb-8 drop-shadow"/>
                <div class="flex flex-col items-center text-6xl z-10">
                    <h1>{"Arrêtez de vous les briser,"}</h1>
                    <h1>{"utilisez Tournoix"}</h1>
                </div>
                <div class="text-2xl mt-5 z-10">
                    {"Tournoix vous permet de gérer vos tournois et de garder vos spectateurs engagés."}
                </div>
                
                <div class="mt-16 text-2xl">
                    <Button label="Créer un tournoi maintenant" {onclick}/>
                </div>
            </div>
        </HomeLayout>
    }
}