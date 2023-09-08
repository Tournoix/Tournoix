use std::str::FromStr;

use time::Duration;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yew_notifications::use_notification;
use yew_router::prelude::use_navigator;

use crate::{
    api::{self, models::Tournament, tournoix::UpdateTournoixRequest},
    components::{
        backlink::Backlink,
        bracket::{Bracket, Match},
        button::Button,
        form_input::FormInput,
        groups::Groups,
        join_code::JoinCode,
        loading_circle::LoadingCircle,
        qualification_phase::QualificationPhase,
        teams::Teams, results::Results,
    },
    layouts::homelayout::HomeLayout,
    notification::{CustomNotification, NotifType},
    routers::Route,
};

#[derive(PartialEq, Properties)]
pub struct TournoixEditProps {
    pub id: i32,
}

#[function_component]
pub fn TournoixEdit(props: &TournoixEditProps) -> Html {
    let TournoixEditProps { id } = props;
    let navigator = use_navigator().unwrap();
    let tournament: UseStateHandle<Option<Tournament>> = use_state(|| None);
    let loading = use_state(|| true);
    let should_update = use_state(|| false);
    let notifs = use_notification::<CustomNotification>();
    let trigger = use_state(|| false);

    // Form inputs
    let name_ref = use_node_ref();
    let date_ref = use_node_ref();
    let location_ref = use_node_ref();
    let description_ref = use_node_ref();
    let is_qualif = use_state(|| false);
    let is_elim = use_state(|| false);

    let tournament_is_started = use_state(|| false);

    let on_qualif_change = {
        let is_qualif = is_qualif.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();

            is_qualif.set(target.unchecked_into::<HtmlInputElement>().checked());
        })
    };

    let on_elim_change = {
        let is_elim = is_elim.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();

            is_elim.set(target.unchecked_into::<HtmlInputElement>().checked());
        })
    };

    {
        let tournament = tournament.clone();
        let tournament_clone2 = tournament.clone();
        let loading = loading.clone();
        let id = id.clone();
        let is_qualif = is_qualif.clone();
        let is_elim = is_elim.clone();
        let notifs = notifs.clone();
        let tournament_is_started = tournament_is_started.clone();
        let tournament_is_started_clone = tournament_is_started.clone();

        use_effect_with_deps(
            move |_| {
                let tournament = tournament.clone();
                let tournament_clone = (*tournament).clone();
                let tournament_is_started = tournament_is_started.clone();
                spawn_local(async move {
                    let tournoix = match api::tournoix::get(id).await {
                        Ok(t) => Some(t),
                        Err(e) => {
                            notifs.spawn(CustomNotification::new(
                                &format!("Erreur: {}", e.error.reason),
                                &e.error.description,
                                NotifType::Error,
                                Duration::seconds(5),
                            ));

                            None
                        }
                    };

                    if let Some(tournoix) = &tournoix {
                        if let Some(is_started) = api::tournoix::is_tournoix_started(tournoix.id.clone()).await.ok() {
                            tournament_is_started.set(is_started);
                        }

                        is_elim.set(tournoix.is_elim);
                        is_qualif.set(tournoix.is_qualif);
                    }

                    tournament.set(tournoix);
                    loading.set(false);
                });

                || ()
            },
            trigger.clone(),
        );
    }

    let on_click_view = {
        let navigator = navigator.clone();
        let id = id.clone();
        Callback::from(move |_| navigator.push(&Route::TournoixView { id }))
    };

    let on_submit = {
        let tournament = tournament.clone();

        let name_ref = name_ref.clone();
        let date_ref = date_ref.clone();
        let location_ref = location_ref.clone();
        let description_ref = description_ref.clone();
        // let groupe_size_ref = groupe_size_ref.clone();
        let is_qualif = is_qualif.clone();
        let is_elim = is_elim.clone();
        let trigger = trigger.clone();
        let notifs = notifs.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let name = name_ref.cast::<HtmlInputElement>().unwrap().value();
            let date = date_ref.cast::<HtmlInputElement>().unwrap().value();
            let location = location_ref.cast::<HtmlInputElement>().unwrap().value();
            let description = description_ref.cast::<HtmlInputElement>().unwrap().value();
            // let groupe_size = groupe_size_ref.cast::<HtmlInputElement>().unwrap().value();
            // let qualif = qualif_ref.cast::<HtmlInputElement>().unwrap().checked();
            // let elim = elim_ref.cast::<HtmlInputElement>().unwrap().checked();

            /*
            let groupe_size = match groupe_size.is_empty() {
                true => None,
                false => Some(groupe_size.parse().unwrap()),
            };
            */

            let date = chrono::NaiveDateTime::from_str(&format!("{}:00", date)).unwrap();

            let update_request = UpdateTournoixRequest {
                name: Some(name),
                date: Some(date),
                location: Some(location),
                description: Some(description),
                phase: None,
                size_group: None,
                is_qualif: Some(*is_qualif),
                is_elim: Some(*is_elim),
                is_closed: Some(false)
            };

            {
                let tournament = tournament.clone();
                let notifs = notifs.clone();
                let trigger = trigger.clone();

                spawn_local(async move {
                    match tournament.as_ref().unwrap().update(update_request).await {
                        Ok(tournament) => {
                            notifs.spawn(CustomNotification::new(
                                "Modification réussie",
                                &format!(
                                    "Vous avez modifié avec succès le tournoi \"{}\"",
                                    tournament.name
                                ),
                                NotifType::Success,
                                Duration::seconds(5),
                            ));
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

                    trigger.set(!*trigger);
                });
            }
        })
    };

    let on_teams_update = {
        let should_update = should_update.clone();
        Callback::from(move |_| {
            should_update.set(!*should_update);
        })
    };

    let on_qualif_gen_click = {
        let tournament = tournament.clone();
        let should_update = should_update.clone();

        Callback::from(move |_| {
            if tournament.is_some() {
                let tournament = tournament.clone();
                let should_update = should_update.clone();

                spawn_local(async move {
                    let _ = tournament.as_ref().unwrap().generate_qualif_games().await;
                    should_update.set(!*should_update);
                });
            }
        })
    };

    let on_qualif_reset_click = {
        let tournament = tournament.clone();
        let should_update = should_update.clone();
        let notifs = notifs.clone();

        Callback::from(move |_| {
            if tournament.is_some() {
                let tournament = tournament.clone();
                let should_update = should_update.clone();
                let notifs = notifs.clone();

                spawn_local(async move {
                    match tournament.as_ref().unwrap().reset_qualif_games().await {
                        Ok(_) => {},
                        Err(e) => {
                            notifs.spawn(CustomNotification::new(
                                &format!("Erreur: {}", e.error.reason),
                                &e.error.description,
                                NotifType::Error,
                                Duration::seconds(5),
                            ));
                        }
                    };

                    should_update.set(!*should_update);
                });
            }
        })
    };

    let on_elim_gen_click = {
        let tournament = tournament.clone();
        let should_update = should_update.clone();
        let notifs = notifs.clone();

        Callback::from(move |_| {
            if tournament.is_some() {
                let tournament = tournament.clone();
                let should_update = should_update.clone();
                let notifs = notifs.clone();

                spawn_local(async move {
                    match tournament.as_ref().unwrap().generate_elim_games().await {
                        Ok(_) => {},
                        Err(e) => {
                            notifs.spawn(CustomNotification::new(
                                &format!("Erreur: {}", e.error.reason),
                                &e.error.description,
                                NotifType::Error,
                                Duration::seconds(5),
                            ));
                        }
                    };

                    should_update.set(!*should_update);
                });
            }
        })
    };

    let on_elim_reset_click = {
        let tournament = tournament.clone();
        let should_update = should_update.clone();
        let notifs = notifs.clone();

        Callback::from(move |_| {
            if tournament.is_some() {
                let tournament = tournament.clone();
                let should_update = should_update.clone();
                let notifs = notifs.clone();

                spawn_local(async move {
                    match tournament.as_ref().unwrap().reset_elim_games().await {
                        Ok(_) => {},
                        Err(e) => {
                            notifs.spawn(CustomNotification::new(
                                &format!("Erreur: {}", e.error.reason),
                                &e.error.description,
                                NotifType::Error,
                                Duration::seconds(5),
                            ));
                        }
                    };

                    should_update.set(!*should_update);
                });
            }
        })
    };

    let close_tournament_click = {
        let tournament = tournament.clone();
        let notifs = notifs.clone();
        let trigger = trigger.clone();

        Callback::from(move |_| {
            if tournament.is_some() {
                let tournament = tournament.clone();
                let notifs = notifs.clone();
                let trigger = trigger.clone();

                spawn_local(async move {
                    match tournament.as_ref().unwrap().close().await {
                        Ok(_) => {
                            notifs.spawn(CustomNotification::new(
                                "Tournoi fermé !",
                                &format!("Le tournoi \"{}\" à été fermé", tournament.as_ref().unwrap().name),
                                NotifType::Success,
                                Duration::seconds(5),
                            ));
                            trigger.set(!*trigger);
                        },
                        Err(e) => {
                            notifs.spawn(CustomNotification::new(
                                &format!("Erreur: {}", e.error.reason),
                                &e.error.description,
                                NotifType::Error,
                                Duration::seconds(5),
                            ));
                        }
                    };
                });
            }
        })
    };

    html! {
        <HomeLayout>
            <div class="flex flex-col items-center h-full pb-16 pt-12 sm:w-9/12 w-11/12 mx-auto relative">
                <Backlink route={Route::Tournoix} label="Retour à la liste des tournoix"/>
                if *loading {
                    <LoadingCircle />
                } else {
                    if let Some(tournament) = &*tournament {
                        <h1 class="mb-5">{format!("Modification de \"{}\"", {tournament.name.to_string()})}</h1>
                        <a onclick={on_click_view} class="a_link mb-6">{"Voir ce tournoi en mode affichage"}</a>
                        <JoinCode code={tournament.code.to_string()}/>
                        <hr/>
                        <h2>{"Informations"}</h2>
                        if tournament.is_closed {
                            <div class="text-lg">{"Etat: Ce tournoi est fermé."}</div>
                        } else {
                            if *tournament_is_started {
                                <div class="text-lg">{"Etat: Ce tournoi a démarré."}</div>
                            } else {
                                <div class="text-lg">{"Etat: Ce tournoi n'est pas encore démarré."}</div>
                            }
                        }
                        <div class="flex flex-row w-full justify-center gap-5 lg:flex-nowrap flex-wrap">
                            <div class="w-1/2">
                                <form class="flex flex-col items-end" onsubmit={on_submit}>
                                    <FormInput id="name" label="Nom" form_type="text" value={tournament.name.clone()} _ref={name_ref} required={true}/>
                                    <FormInput id="date" label="Date" form_type="datetime-local" value={tournament.date.format("%Y-%m-%dT%H:%M").to_string()}  _ref={date_ref} required={true}/>
                                    <FormInput id="location" label="Lieu" form_type="text" value={tournament.location.as_ref().unwrap_or(&String::new()).to_string()}  _ref={location_ref} required={true}/>
                                    <FormInput id="description" label="Description" form_type="text" value={tournament.description.clone()}  _ref={description_ref} required={true}/>
                                    // <FormInput id="nb_team_per_group" label="Nombre d'équipes par groupe" form_type="number" value={if let Some(s) = tournament.size_group {s.to_string()} else {String::new()}}  _ref={groupe_size_ref}/>
                                    <FormInput id="phase_qualifications" disabled={*tournament_is_started} label="Phase de qualifications" form_type="checkbox" checked={*is_qualif} onchange={on_qualif_change} />
                                    <FormInput id="phase_eliminations" disabled={*tournament_is_started} label="Phase d'éliminations" form_type="checkbox" checked={*is_elim} onchange={on_elim_change} />

                                    <Button class="text-lg px-3 py-2 mt-3 hover:scale-110 bg-green-700">{"Sauvegarder les informations"}</Button>
                                </form>
                            </div>
                            <div class="w-1/2 m-4">
                                <Teams tournament={tournament.clone()} on_update={on_teams_update} />
                            </div>
                        </div>
                        if *is_qualif {
                            <hr/>
                            <h2>{"Phase de qualifications"}</h2>
                            <Groups tournament={tournament.clone()} should_update={should_update.clone()} editable={true} />
                            <div class={"flex gap-4 mt-3"}>
                                <Button class="text-lg px-3 py-2 hover:scale-110 bg-green-700" onclick={on_qualif_gen_click}>{"Générer les matches"}</Button>
                                <Button class="text-lg px-3 py-2 hover:scale-110 bg-green-700" onclick={on_qualif_reset_click}>{"Réinitialiser les matches"}</Button>
                            </div>
                            <QualificationPhase tournament={tournament.clone()} should_update={should_update.clone()} editable={true} />
                        }
                        if *is_elim {
                            <hr/>
                            <h2>{"Phase d'éliminations"}</h2>
                            
                            <div class={"flex gap-4 mt-3 mb-3"}>
                                <Button class="text-lg px-3 py-2 hover:scale-110 bg-green-700" onclick={on_elim_gen_click}>{"Générer les matches"}</Button>
                                <Button class="text-lg px-3 py-2 hover:scale-110 bg-green-700" onclick={on_elim_reset_click}>{"Réinitialiser les matches"}</Button>
                            </div>
                            <Bracket tournament={tournament.clone()} should_update={should_update} editable={true} />
                        }
                        <hr/>
                        <h2>{"Résultats"}</h2>
                        <Results tournament_id={ id } can_show_results={tournament.is_closed.clone() && *tournament_is_started} />
                        if !tournament.is_closed.clone() {
                            <Button disabled={!*tournament_is_started} onclick={close_tournament_click} class="mt-6 px-3 py-2 hover:scale-110">{"Fermer le tournoi"}</Button>
                        }
                    } else {
                        <div>{"Oups, ce tournoi n'existe pas :("}</div>
                    }
                }
            </div>
        </HomeLayout>
    }
}
