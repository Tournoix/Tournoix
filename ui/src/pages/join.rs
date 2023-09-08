use time::Duration;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yew_notifications::use_notification;
use yew_router::prelude::use_navigator;

use crate::{
    api::{self, models::Tournament, tournoix::SubscriptionRequest},
    components::button::Button,
    layouts::homelayout::HomeLayout,
    notification::{CustomNotification, NotifType},
    routers::Route,
};

#[derive(PartialEq, Properties)]
pub struct JoinProps {
    pub code: String,
}

#[function_component]
pub fn Join(props: &JoinProps) -> Html {
    let JoinProps { code } = props;

    let navigator = use_navigator().unwrap();
    let notifs = use_notification::<CustomNotification>();
    let tournament: UseStateHandle<Option<Tournament>> = use_state(|| None);
    let loading = use_state(|| true);

    {
        let tournament = tournament.clone();
        let loading = loading.clone();
        let code = code.clone();

        use_effect_once(move || {
            let tournament = tournament.clone();
            let loading = loading.clone();

            spawn_local(async move {
                tournament.set(api::tournoix::get_by_code(code).await.ok());
                loading.set(false);
            });

            || ()
        });
    }

    let on_click = {
        let navigator = navigator.clone();
        let tournament = tournament.clone();
        let code = code.clone();
        let notifs = notifs.clone();

        Callback::from(move |_| {
            let navigator = navigator.clone();
            let notifs = notifs.clone();
            let code = code.clone();
            let tournament = tournament.clone();

            spawn_local(async move {
                match api::tournoix::subscribe(SubscriptionRequest { code }).await {
                    Ok(_) => {
                        navigator.push(&Route::TournoixView {
                            id: tournament.as_ref().unwrap().id,
                        });
                    }

                    Err(e) => {
                        notifs.spawn(CustomNotification::new(
                            &format!("Erreur: {}", e.error.reason),
                            &e.error.description,
                            NotifType::Error,
                            Duration::seconds(5),
                        ));
                    }
                }
            });
        })
    };

    html! {
        <HomeLayout>
            <div class="flex flex-col items-center h-full pb-16 pt-12 sm:w-9/12 w-11/12 mx-auto relative">
                if let Some(tournament) = &*tournament {
                    <h1 class="mb-5">{"Rejoindre: "}{tournament.name.clone()}</h1>
                    <Button onclick={on_click} class="flex items-center gap-2 sm:text-xl text-lg px-3 py-2 mx-auto mt-3 mb-16 hover:scale-110 bg-green-700">
                        {"Rejoindre le tournoi"}
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M11.25 4.5l7.5 7.5-7.5 7.5m-6-15l7.5 7.5-7.5 7.5" />
                        </svg>
                    </Button>
                } else {
                    <div>{"Oops, ce tournoi n'existe pas :("}</div>
                }
            </div>
        </HomeLayout>
    }
}
