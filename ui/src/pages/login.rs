use time::Duration;

use yew::prelude::*;
use yew_router::prelude::use_navigator;
use web_sys::console;

use crate::routers::Route;
use crate::components::{button::Button, form_input::FormInput};
use crate::layouts::homelayout::HomeLayout;
use web_sys::window;
use yew_notifications::{use_notification, Notification, NotificationType};

#[derive(PartialEq, Properties)]
pub struct LoginProps {}

#[function_component]
pub fn Login(props: &LoginProps) -> Html {
    let LoginProps {} = props;
    let navigator = use_navigator().unwrap();
    let notifications_manager = use_notification::<Notification>();

    let on_test_click = {
        Callback::from(move |_| {
            console::log_1(&"Hello from Yew!".into());
            notifications_manager.spawn(Notification::new(
                NotificationType::Warn,
                "Connexion",
                "yoyoyo",
                Duration::seconds(999)
            ));
        })
    };

    let on_login_click: Callback<MouseEvent> = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            // TODO login
            if let Some(win) = window() {
                if let Ok(Some(store)) = win.local_storage() {
                    if let Ok(_item) = store.set_item("loginToken", "temp") { }
                }
            }

            /*
            notifications_manager.spawn(Notification::new(
                NotificationType::Info,
                "Connexion",
                "Connexion réussie",
                Duration::seconds(5)
            ));
            */

            //navigator.push(&Route::Home)
        })
    };

    html! {
        <HomeLayout>
            <div class="flex flex-col items-center h-full sm:w-9/12 w-11/12 mx-auto relative">
                <Button class="px-3 py-2 mx-auto mt-3 mb-16 hover:scale-110" onclick={on_test_click}>{"TEST"}</Button>
                <h1 class="mt-12 mb-5">{"Connexion"}</h1>
                <form class="flex flex-col sm:text-xl text-lg" id="login-form">
                    <FormInput id="email" label="E-mail" form_type="email" required={true}/>
                    <FormInput id="password" label="Mot de passe" form_type="password" required={true}/>
                    <div class="mx-auto"><FormInput id="remember" label="Se rappeler de moi" form_type="checkbox" required={false}/></div>
                    <Button class="px-3 py-2 mx-auto mt-3 mb-16 hover:scale-110" onclick={on_login_click}>{"Connexion"}</Button>
                </form>
            </div>
        </HomeLayout>
    }
}
