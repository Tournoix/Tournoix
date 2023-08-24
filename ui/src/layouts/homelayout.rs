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
        <div class="font-bebas bg-[#fbfefb] min-h-full">
            <header class="h-16 bg-nutDark flex items-center drop-shadow-lg z-50 sticky top-0">
                <div class="layout-nav">
                    <a onclick={on_home_click} href="javascript:void" class="flex flex-row my-auto transition-all hover:tracking-[.2em] hover:scale-[105%] origin-left hover:duration-[200ms] duration-[400ms]">
                        <img src="/img/nut_invert.png" class="sm:h-12 h-8 sm:mr-8 mr-2"/>
                        <div>
                            <h1 class="sm:text-5xl text-3xl text-white">{"Tournoix"}</h1>
                        </div>
                    </a>
                    <div class="ml-auto my-auto flex">
                        <Button class="px-4 py-1 origin-right hover:scale-110" onclick={on_login_click}>{"Connexion"}</Button>
                    </div>
                </div>
            </header>

            <main class={"w-full"}>
                {children.clone()}
            </main>

            <footer class="sticky bg-nutDark w-full">
                <div class="layout-nav">
                    <div class="relative my-auto">
                        <h3 class="mt-1">{"A propos"}</h3>
                        <ul class="leading-4">
                            <li><p>{"L'équipe"}</p></li>
                            <li><p>{"Contact"}</p></li>
                            <li><p>{"Localisation"}</p></li>
                        </ul>
                    </div>
                    <div class="relative ml-auto text-right my-auto">
                        <img src="/img/nut_invert.png" class="h-8 ml-auto"/>
                        <p>{"Copyright Tournoix"}</p>
                        <p>{"© 2023"}</p>
                    </div>
                </div>
            </footer>
        </div>
    }
}
