use std::str::FromStr;

use time::Duration;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_notifications::use_notification;
use yew_router::prelude::use_navigator;

use crate::{
    api::{self, tournoix::CreateTournoixRequest},
    components::{backlink::Backlink, button::Button, form_input::FormInput},
    layouts::homelayout::HomeLayout,
    notification::{CustomNotification, NotifType},
    routers::Route,
};

#[derive(PartialEq, Properties)]
pub struct TournoixCreateProps {}

#[function_component]
pub fn TournoixCreate(props: &TournoixCreateProps) -> Html {
    let TournoixCreateProps {} = props;
    let navigator = use_navigator().unwrap();
    let notifs = use_notification::<CustomNotification>();

    let name_ref = use_node_ref();
    let date_ref = use_node_ref();
    let location_ref = use_node_ref();
    let description_ref = use_node_ref();

    let on_submit = {
        let name_ref = name_ref.clone();
        let date_ref = date_ref.clone();
        let location_ref = location_ref.clone();
        let description_ref = description_ref.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let navigator = navigator.clone();

            let name = name_ref.cast::<HtmlInputElement>().unwrap().value();
            let date = date_ref.cast::<HtmlInputElement>().unwrap().value();
            let location = location_ref.cast::<HtmlInputElement>().unwrap().value();
            let description = description_ref.cast::<HtmlInputElement>().unwrap().value();

            let date = chrono::NaiveDateTime::from_str(&format!("{}:00", date)).unwrap();

            let create_request = CreateTournoixRequest {
                name,
                date,
                description,
                location,
                is_qualif: false,
                is_elim: false,
                is_closed: false,
            };

            let notifs = notifs.clone();
            spawn_local(async move {
                match api::tournoix::create(create_request).await {
                    Ok(tournoix) => {
                        notifs.spawn(CustomNotification::new(
                            "Tournoi créé !",
                            &format!("Votre tournoi [{}] à été créé", tournoix.name),
                            NotifType::Success,
                            Duration::seconds(5),
                        ));

                        navigator.push(&Route::TournoixEdit {
                            id: tournoix.id as i32,
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
                <Backlink route={Route::Tournoix} label="Retour à la liste des tournoix"/>
                <h1 class="mb-5">{"Création de tournoi"}</h1>
                <form onsubmit={on_submit} class="flex flex-col items-center w-full mx-auto relative">
                    <h2>{"Informations"}</h2>
                    <div>
                        <FormInput id="name" label="Nom" form_type="text" _ref={name_ref} required={true}/>
                        <FormInput id="date" label="Date" form_type="datetime-local" _ref={date_ref} required={true}/>
                        <FormInput id="location" label="Lieu" form_type="text" _ref={location_ref} required={true}/>
                        <FormInput id="description" label="Description" form_type="text" _ref={description_ref} required={true}/>
                    </div>
                    <Button class="flex items-center gap-2 sm:text-xl text-lg px-3 py-2 mx-auto mt-3 mb-16 hover:scale-110 bg-green-700">
                        {"Créer un tournoi"}
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M11.25 4.5l7.5 7.5-7.5 7.5m-6-15l7.5 7.5-7.5 7.5" />
                        </svg>
                    </Button>
                </form>
            </div>
        </HomeLayout>
    }
}
