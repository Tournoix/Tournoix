use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_notifications::{NotificationsProvider, Notification, NotificationFactory, NotificationsPosition};
use yew_router::prelude::use_navigator;
use time::Duration;

use crate::{components::button::Button, routers::Route};
use web_sys::window;
use yew_notifications::{use_notification, NotificationType};

#[derive(PartialEq, Properties)]
pub struct HomeLayoutProps {
    pub children: Children,
}

#[function_component]
pub fn HomeLayout(props: &HomeLayoutProps) -> Html {
    let HomeLayoutProps { children } = props;
    let navigator = use_navigator().unwrap();
    let is_logged = use_state(|| false);
    let component_creator = NotificationFactory::default();
    let notifications_manager = use_notification::<Notification>();

    {
        let is_logged = is_logged.clone();
        let notifications_manager = notifications_manager.clone();
        use_effect_once(move || {
            if let Some(win) = window() {
                if let Ok(Some(store)) = win.local_storage() {
                    if let Ok(Some(_item)) = store.get_item("loginToken") {
                        is_logged.set(true);
                    }
                }
            }
            notifications_manager.spawn(Notification::new(
                NotificationType::Warn,
                "Connexion",
                "yoyoyo",
                Duration::seconds(999)
            ));

            || {}
        });
    }

    let on_test_click = {
        Callback::from(move |_| {
            notifications_manager.spawn(Notification::new(
                NotificationType::Warn,
                "Connexion",
                "yoyoyo",
                Duration::seconds(5)
            ));
        })
    };

    let on_home_click = {
        let navigator = navigator.clone();
        
        Callback::from(move |_| navigator.push(&Route::Home))
    };

    let on_login_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Login))
    };

    let on_logout_click = {
        let navigator = navigator.clone();
        let is_logged = is_logged.clone();
        Callback::from(move |_| {
            // TODO logout

            if let Some(win) = window() {
                if let Ok(Some(store)) = win.local_storage() {
                    if let Ok(_item) = store.remove_item("loginToken") {
                        is_logged.set(false);
                    }
                }
            }

            navigator.push(&Route::Home)
        })
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
                    if *is_logged {
                        <div class="ml-auto my-auto flex">
                            <Button class="sm:px-4 px-2 py-1 origin-right hover:scale-110 sm:text-base text-sm" onclick={on_logout_click}>{"Déconnexion"}</Button>
                        </div>
                    } else {
                        <div class="ml-auto my-auto flex">
                            <Button class="sm:px-4 px-2 py-1 origin-right hover:scale-110 sm:text-base text-sm" onclick={on_login_click}>{"Connexion"}</Button>
                        </div>
                    }
                </div>
            </header>
                
                <div>
                <NotificationsProvider<Notification, NotificationFactory> {component_creator} position={NotificationsPosition::TopRight}>
                    <button onclick={on_test_click}>{"test"}</button>
                </NotificationsProvider<Notification, NotificationFactory>>
                <div style="height: 150vh"></div>
                </div>
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
