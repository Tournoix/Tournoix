use log::info;
use yew::platform::spawn_local;
use yew::prelude::*;

use crate::components::loading_circle::LoadingCircle;
use crate::components::tournament_create_button::TournamentCreateButton;
use crate::components::tournaments::Tournaments;
use crate::components::user_provider::UserContext;
use crate::layouts::homelayout::HomeLayout;

#[derive(PartialEq, Properties)]
pub struct TournoixProps {}

#[function_component]
pub fn Tournoix(props: &TournoixProps) -> Html {
    let TournoixProps {} = props;
    let user_info = use_context::<UserContext>().expect("Missing user context provider");
    let owned_tournaments = use_state(|| Vec::new());
    let joined_tournaments = use_state(|| Vec::new());
    let loading_owned = use_state(|| true);
    let loading_joined = use_state(|| true);
    let trigger = use_state(|| false);

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
                            //info!("{}", tournoix);
                            joined_tournaments.set(tournoix);
                        }

                        loading_joined.set(false);
                    });
                }

                || ()
            },
            (user_info, trigger.clone()),
        );
    }

    let on_tournament_delete = Callback::from(move |_| {
        info!("on delete");
        trigger.set(!*trigger);
    });

    html! {
        <HomeLayout>
            <div class="flex flex-col items-center h-full pb-16 sm:w-9/12 w-11/12 mx-auto relative">
                <h1 class="mt-12 mb-5">{"Liste des tournoix"}</h1>
                <h2 class="mt-12 mb-5">{"Mes tournoix"}</h2>
                if *loading_owned {
                    <LoadingCircle />
                } else {
                    <div class="flex gap-4 w-full mt-2 justify-center">
                        <TournamentCreateButton />
                        <Tournaments tournaments={(*owned_tournaments).clone()} editable={true} on_delete={on_tournament_delete} />
                    </div>
                }
                <h2 class="mt-12 mb-5">{"Tournoix rejoints"}</h2>
                if *loading_joined {
                    <LoadingCircle />
                } else {
                    if joined_tournaments.len() == 0 {
                        <span>{"Vous n'avez rejoint aucun tournois"}</span>
                    } else {
                        <Tournaments tournaments={(*joined_tournaments).clone()} />
                    }
                }
            </div>
        </HomeLayout>
    }
}
