use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::components::loading_circle::LoadingCircle;
use crate::components::tournaments::Tournaments;
use crate::components::user_provider::UserContext;
use crate::layouts::homelayout::HomeLayout;
use crate::routers::Route;

#[derive(PartialEq, Properties)]
pub struct TournoixProps {}

#[function_component]
pub fn Tournoix(props: &TournoixProps) -> Html {
    let TournoixProps {} = props;
    let navigator = use_navigator().unwrap();
    let user_info = use_context::<UserContext>().expect("Missing user context provider");
    let owned_tournaments = use_state(|| Vec::new());
    let joined_tournaments = use_state(|| Vec::new());
    let loading_owned = use_state(|| true);
    let loading_joined = use_state(|| true);

    // TODO: Should move component based features inside these components
    let on_create_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::TournoixCreate))
    };

    let on_edit_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::TournoixEdit { id: 42 }))
    };
    let on_delete_click = Callback::from(move |_| {
        if !gloo_dialogs::confirm("Êtes-vous sûr de vouloir supprimer ce tournoi ?") {
            return;
        }
    });
    let on_leave_click = Callback::from(move |_| {
        if !gloo_dialogs::confirm("Êtes-vous sûr de vouloir quitter ce tournoi ?") {
            return;
        }
    });

    {
        let owned_tournaments = owned_tournaments.clone();
        let joined_tournaments = joined_tournaments.clone();
        let user_info = user_info.clone();
        let user = user_info.user.clone();
        let loading_owned = loading_owned.clone();
        let loading_joined = loading_joined.clone();

        use_effect_with_deps(
            move |_| {
                if let Some(user) = user {
                    {
                        let user = user.clone();

                        spawn_local(async move {
                            if let Some(tournoix) = user.tournaments().await.ok() {
                                owned_tournaments.set(tournoix);
                            }
                            loading_owned.set(false);
                        });
                    }

                    spawn_local(async move {
                        if let Some(tournoix) = user.subscriptions().await.ok() {
                            joined_tournaments.set(tournoix);
                        }

                        loading_joined.set(false);
                    });
                }

                || ()
            },
            user_info,
        );
    }

    html! {
        <HomeLayout>
            <div class="flex flex-col items-center h-full pb-16 sm:w-9/12 w-11/12 mx-auto relative">
                <h1 class="mt-12 mb-5">{"Liste des tournoix"}</h1>
                <h2 class="mt-12 mb-5">{"Mes tournoix"}</h2>
                if *loading_owned {
                    <LoadingCircle />
                } else {
                    <Tournaments tournaments={(*owned_tournaments).clone()} on_create={on_create_click} on_delete={on_delete_click} on_edit={on_edit_click}/>
                }
                <h2 class="mt-12 mb-5">{"Tournoix rejoints"}</h2>
                if *loading_joined {
                    <LoadingCircle />
                } else {
                    if joined_tournaments.len() == 0 {
                        <span>{"Vous n'avez rejoint aucun tournois"}</span>
                    } else {
                        <Tournaments tournaments={(*joined_tournaments).clone()} nb_nuts={42} on_leave={on_leave_click}/>
                    }
                }
            </div>
        </HomeLayout>
    }
}
