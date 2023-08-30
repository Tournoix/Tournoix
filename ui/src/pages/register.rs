use serde::{Serialize, Deserialize};
use web_sys::HtmlInputElement;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use dotenv_codegen::dotenv;

use crate::components::{form_input::FormInput, button::Button};
use crate::layouts::homelayout::HomeLayout;
use crate::routers::Route;

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

            let register_request = RegisterForm {
                name: username,
                email,
                password
            };

            {
                let navigator = navigator.clone();
                let loading = loading.clone();

                spawn_local(async move {
                    let client = reqwest::Client::new();
    
                    match client.post(format!("{}/{}", dotenv!("API_ENDPOINT"), "auth/register"))
                        .body(serde_json::to_string(&register_request).unwrap())
                        .send()
                        .await {
                            Ok(r) => {
                                match r.error_for_status_ref() {
                                    Ok(_r) => {
                                        let _response = r.json::<RegisterResponse>().await.unwrap();
                                        
                                        loading.set(false);
                                        navigator.push(&Route::Login);
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
