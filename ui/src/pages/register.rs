use serde::{Serialize, Deserialize};
use web_sys::HtmlInputElement;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::api::auth::RegisterRequest;
use crate::api::{self};
use crate::components::notification::NotifType;
use crate::components::{form_input::FormInput, button::Button};
use crate::layouts::homelayout::HomeLayout;
use crate::routers::Route;
use crate::utils::utils::add_delayed_notif;

#[derive(Serialize, Default)]
pub struct RegisterForm {
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct RegisterResponse {
    pub id: i32,
    pub name: String,
    pub email: String
}

#[derive(PartialEq, Properties)]
pub struct RegisterProps {}

#[function_component]
pub fn Register(props: &RegisterProps) -> Html {
    let RegisterProps {} = props;
    let navigator = use_navigator().unwrap();
    let username_ref = use_node_ref();
    let email_ref = use_node_ref();
    let password_ref = use_node_ref();
    let loading = use_state(|| false);
    
    let on_register_submit = {
        let navigator = navigator.clone();
        let username_ref = username_ref.clone();
        let email_ref = email_ref.clone();
        let password_ref = password_ref.clone();
        let loading = loading.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            loading.set(true);
            
            let username = username_ref.cast::<HtmlInputElement>().unwrap().value();
            let email = email_ref.cast::<HtmlInputElement>().unwrap().value();
            let password = password_ref.cast::<HtmlInputElement>().unwrap().value();

            let register_request = RegisterRequest {
                name: username,
                email,
                password
            };

            {
                let navigator = navigator.clone();
                let loading = loading.clone();

                spawn_local(async move {
                    match api::auth::register(register_request).await {
                        Ok(_) => {
                            loading.set(false);

                            add_delayed_notif(
                                "Compte créé !",
                                "Votre compte à été créé !",
                                NotifType::Success,
                            );

                            navigator.push(&Route::Login);
                        },

                        Err(e) => {
                            loading.set(false);

                            add_delayed_notif(
                                "Erreur",
                                &e.error.description,
                                NotifType::Error,
                            );
                        }
                    }
                });
            }
        })
    };

    html! {
        <HomeLayout>
            <div class="flex flex-col items-center h-full sm:w-9/12 w-11/12 mx-auto relative">
                <h1 class="mt-12 mb-5">{"Créer un compte"}</h1>
                <form class="flex flex-col sm:text-xl text-lg" id="login-form" onsubmit={on_register_submit}>
                    <FormInput id="email" label="E-mail" form_type="email" required={true} _ref={email_ref}/>
                    <FormInput id="username" label="Nom d'utilisateur" form_type="text" required={true} _ref={username_ref}/>
                    <FormInput id="password" label="Mot de passe" form_type="password" required={true} _ref={password_ref}/>
                    <Button class={classes!("px-3", "py-2", "mx-auto", "mt-3", "mb-16", if *loading {"animate-pulse"} else {"hover:scale-110"})} disabled={*loading}>{"Créer un compte"}</Button>
                </form>
            </div>
        </HomeLayout>
    }
}
