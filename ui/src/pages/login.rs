use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use dotenv_codegen::dotenv;
use serde::{Serialize, Deserialize};

use crate::components::notification::NotifType;
use crate::components::user_provider::UserContext;
use crate::routers::Route;
use crate::components::{button::Button, form_input::FormInput};
use crate::layouts::homelayout::HomeLayout;
use crate::utils::utils::*;
use web_sys::HtmlInputElement;

#[derive(PartialEq, Properties)]
pub struct LoginProps {}

#[derive(Serialize, Default)]
pub struct LoginForm {
    pub email: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub expiration_date: chrono::NaiveDateTime
}

#[function_component]
pub fn Login(props: &LoginProps) -> Html {
    let LoginProps {} = props;
    let navigator = use_navigator().unwrap();
    let email_ref = use_node_ref();
    let password_ref = use_node_ref();
    let loading = use_state(|| false);
    let user_info = use_context::<UserContext>().expect("Missing UserInfo contect provider");

    let on_login_submit = {
        let navigator = navigator.clone();
        let email_ref = email_ref.clone();
        let password_ref = password_ref.clone();
        let loading = loading.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            loading.set(true);
            
            let email = email_ref.cast::<HtmlInputElement>().unwrap().value();
            let password = password_ref.cast::<HtmlInputElement>().unwrap().value();

            let login_request = LoginForm {
                email,
                password
            };

            {
                let navigator = navigator.clone();
                let loading = loading.clone();
                let user_info = user_info.clone();

                spawn_local(async move {
                    let client = reqwest::Client::new();

                    match client.post(format!("{}/{}", dotenv!("API_ENDPOINT"), "auth/login"))
                        .body(serde_json::to_string(&login_request).unwrap())
                        .send()
                        .await {
                            Ok(r) => {
                                match r.error_for_status_ref() {
                                    Ok(_r) => {
                                        let response = r.json::<LoginResponse>().await.unwrap();

                                        user_info.login(&response.token);
                                        
                                        loading.set(false);
                                        add_delayed_notif("Connecté(e)", "Vous vous êtes connecté(e) avec succès à votre compte.", NotifType::Success);
                                        navigator.push(&Route::Home);
                                    },

                                    Err(_e) => {
                                        loading.set(false);
                                    }
                                }
                            },
    
                            Err(_e) => {
                                loading.set(false);
                            }
                    }
                });
            }
        })
    };

    let on_register_click: Callback<MouseEvent> = {
        let navigator = navigator.clone();
        Callback::from(move |_| {

            navigator.push(&Route::Register)
        })
    };

    html! {
        <HomeLayout>
            <div class="flex flex-col items-center h-full sm:w-9/12 w-11/12 mx-auto relative">
                <h1 class="mt-12 mb-5">{"Connexion"}</h1>
                <form class="flex flex-col sm:text-xl text-lg" id="login-form" onsubmit={on_login_submit}>
                    <FormInput id="email" label="E-mail" form_type="email" required={true} _ref={email_ref.clone()} />
                    <FormInput id="password" label="Mot de passe" form_type="password" required={true} _ref={password_ref.clone()} />
                    <Button class={classes!("px-3", "py-2", "mx-auto", "mt-3", "mb-4", if *loading {"animate-pulse"} else {"hover:scale-110"})} disabled={*loading}>{"Connexion"}</Button>
                </form>
                <Button class="sm:text-xl text-lg px-3 py-2 mx-auto mt-3 mb-16 hover:scale-110" onclick={on_register_click}>{"Créer un compte"}</Button>
            </div>
        </HomeLayout>
    }
}
