use log::info;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::use_navigator;

use crate::api;
use crate::components::notification::{Notif, NotifType, Notification};
use crate::components::user_provider::UserContext;
use crate::{components::button::Button, routers::Route, utils::utils::*};
use web_sys::window;

#[derive(PartialEq, Properties)]
pub struct HomeLayoutProps {
    pub children: Children,
}

#[function_component]
pub fn HomeLayout(props: &HomeLayoutProps) -> Html {
    let HomeLayoutProps { children } = props;
    let navigator = use_navigator().unwrap();
    let is_logged = use_state(|| false);
    let notifs: UseStateHandle<Vec<Notif>> = use_state(|| Vec::new());
    let user_info = use_context::<UserContext>().expect("Missing UserInfo contect provider");

    {
        let is_logged = is_logged.clone();
        use_effect_once(move || {
            if let Some(win) = window() {
                if let Ok(Some(store)) = win.local_storage() {
                    if let Ok(Some(_item)) = store.get_item("loginToken") {
                        is_logged.set(true);
                    }
                }
            }

            || {}
        });
    }

    {
        let notifs = notifs.clone();
        use_effect(move || {
            if let Some(fetched_notifs) = consume_notifs() {

                let mut buf_notifs: Vec<Notif> = vec![];
                let mut curr_id = 0;

                for fetched_notif in fetched_notifs.iter() {
                    buf_notifs.push(Notif {
                        id: curr_id,
                        title: fetched_notif.title.to_string(),
                        content: fetched_notif.content.to_string(),
                        type_notif: fetched_notif.type_notif,
                    });
                    curr_id += 1;
                }
    
                if !buf_notifs.is_empty() {
                    notifs.set(buf_notifs);
                }
            }
        });
    }

    let on_home_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Home))
    };

    let on_login_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Login))
    };

    let on_tournoix_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Tournoix))
    };

    let on_logout_click = {
        let navigator = navigator.clone();
        let is_logged = is_logged.clone();
        
        Callback::from(move |_| {
            let user_info = user_info.clone();
            let is_logged = is_logged.clone();
            let navigator = navigator.clone();

            spawn_local(async move {
                api::logout().await;

                add_delayed_notif("Déconnecté(e)", "Vous vous êtes déconnecté(e) avec succès de votre compte.", NotifType::Success);

                if let Some(win) = window() {
                    if let Ok(Some(store)) = win.local_storage() {
                        if let Ok(_item) = store.remove_item("loginToken") {
                            is_logged.set(false);
                        }
                    }
                }

                navigator.push(&Route::Home)
            });
        })
    };

    html! {
        <div class="font-bebas bg-[#fbfefb] min-h-full">
            <header class="h-16 bg-nutLight flex items-center drop-shadow-lg z-50 sticky top-0">
                <div class="layout-nav">
                    <a onclick={on_home_click} href="javascript:void" class="flex flex-row my-auto transition-all hover:tracking-[.2em] hover:scale-[105%] origin-left hover:duration-[200ms] duration-[400ms]">
                        <img src="/img/nut_invert.png" class="sm:h-12 h-8 sm:mr-8 mr-2"/>
                        <div>
                            <h1 class="sm:text-5xl text-3xl text-white">{"Tournoix"}</h1>
                        </div>
                    </a>
                    if *is_logged {
                        <div class="ml-auto my-auto flex">
                            <Button class="sm:px-4 px-2 py-1 hover:scale-110 sm:text-base text-sm mr-6" onclick={on_tournoix_click}>{"Liste des tournoix"}</Button>
                            <Button class="sm:px-4 px-2 py-1 origin-right hover:scale-110 sm:text-base text-sm" onclick={on_logout_click}>{"Déconnexion"}</Button>
                        </div>
                    } else {
                        <div class="ml-auto my-auto flex">
                            <Button class="sm:px-4 px-2 py-1 origin-right hover:scale-110 sm:text-base text-sm" onclick={on_login_click}>{"Connexion"}</Button>
                        </div>
                    }
                </div>
            </header>

            <main class={"w-full"}>
                <ContextProvider<UseStateHandle<Vec<Notif>>> context={notifs.clone()}>
                    <div id="notifs-container" class="pointer-events-none flex fixed bottom-0 left-0 right-0 sm:w-9/12 w-11/12 h-full z-50 ml-[12.5%] flex-col-reverse items-end pb-12">
                        {
                            notifs.iter().map(|notif| {
                                html!{
                                    <Notification notif={notif.clone()}/>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                    {children.clone()}
                </ContextProvider<UseStateHandle<Vec<Notif>>>>
            </main>

            <footer class="sticky bg-nutLight w-full">
                <div class="layout-nav">
                    <div class="relative my-auto">
                        <h3 class="mt-1">{"A propos"}</h3>
                        <ul class="footer-links">
                            <li><a href="https://heig-vd.ch" >{"HEIG-VD"}</a></li>
                            <li><a href="https://github.com/Tournoix/Tournoix" >{"Code source"}</a></li>
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
