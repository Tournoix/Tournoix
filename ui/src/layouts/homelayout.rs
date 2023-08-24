use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{components::button::Button, routers::Route};

#[derive(PartialEq, Properties)]
pub struct HomeLayoutProps {
    pub children: Children,
}

#[function_component]
pub fn HomeLayout(props: &HomeLayoutProps) -> Html {
    let HomeLayoutProps { children } = props;
    let navigator = use_navigator().unwrap();

    let on_home_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Home))
    };

    let on_login_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Home))
    };

    html! {
        <div class="font-bebas bg-[#fbfefb]">
            <header class="h-16 bg-nutDark flex items-center drop-shadow-lg pl-8 z-50 sticky top-0">
                <a onclick={on_home_click} href="javascript:void" class="flex flex-row ml-5 transition-all hover:tracking-[.2em] hover:duration-[200ms] duration-[400ms]">
                    <img src="/img/nut.svg" class="h-12 mr-8"/>
                    <h1 class="text-5xl text-light">{"Tournoix"}</h1>
                </a>
                <Button class="ml-auto mr-5 px-4 py-1" onclick={on_login_click}>{"Connexion"}</Button>
            </header>

            <main class={"w-full"}>
                {children.clone()}
            </main>

            <footer class="sticky bg-nutDark w-full text-white flex text-lg align-center justify-center pt-3 z-10">
                <div class="relative left-[-40%]">
                    <h3 class="text-2xl mb-1">{"A propos"}</h3>
                    <ul>
                        <li>{"L'équipe"}</li>
                        <li>{"Contact"}</li>
                        <li>{"Localisation"}</li>
                    </ul>
                </div>
                <div class="relative right-[-40%] text-right">
                    <img src="/img/nut.svg" class="h-8 invert ml-auto"/>
                    <p>{"Copyright Tournoix"}</p>
                    <p>{"© 2023"}</p>
                </div>
            </footer>
        </div>
    }
}
