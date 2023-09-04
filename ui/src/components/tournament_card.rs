use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yew_router::prelude::use_navigator;

use crate::{api::models::Tournament, components::loading_circle::LoadingCircle, routers::Route};

#[derive(PartialEq, Properties)]
pub struct TournamentCardProps {
    pub tournament: Tournament,
    #[prop_or_default]
    pub editable: bool,
    #[prop_or_default]
    pub on_delete: Callback<Tournament>
}

#[function_component]
pub fn TournamentCard(props: &TournamentCardProps) -> Html {
    let TournamentCardProps {
        tournament,
        editable,
        on_delete
    } = props;
    let navigator = use_navigator().unwrap();
    let nb_nuts = use_state(|| 0);
    let loading = use_state(|| true);

    {
        let nb_nuts = nb_nuts.clone();
        let tournament = tournament.clone();
        let loading = loading.clone();
        let editable = editable.clone();

        use_effect_once(move || {
            if !editable {
                spawn_local(async move {
                    if let Some(nut) = tournament.get_user_nut().await.ok() {
                        nb_nuts.set(nut.stock)
                    }

                    loading.set(false);
                });
            }

            || ()
        });
    }

    let on_read = {
        let navigator = navigator.clone();
        let tournament = tournament.clone();

        Callback::from(move |_| navigator.push(&Route::TournoixView { id: tournament.id }))
    };

    let on_edit = {
        let navigator = navigator.clone();
        let tournament = tournament.clone();

        Callback::from(move |_| navigator.push(&Route::TournoixEdit { id: tournament.id }))
    };

    let on_delete = {
        let tournament = tournament.clone();
        let on_delete = on_delete.clone();

        Callback::from(move |_| {
            if !gloo_dialogs::confirm("Êtes-vous sûr de vouloir supprimer ce tournoi ?") {
                return;
            }

            let tournament = tournament.clone();
            let on_delete = on_delete.clone();

            spawn_local(async move {
                let _ = tournament.delete().await;
                on_delete.emit(tournament);
            });
        })
    };

    let on_leave = Callback::from(move |_| {
        if !gloo_dialogs::confirm("Êtes-vous sûr de vouloir quitter ce tournoi ?") {
            return;
        }
    });

    html! {
        <div>
            <div class="tournament-card" onclick={on_read.clone()}>{ &tournament.name }</div>
            <div class="tournament-btn-list">

                if *editable {
                    <a onclick={on_edit.clone()}>
                        <img src="/img/pencil.svg" class="tournament-btn-icon hover:bg-orange-400 cursor-pointer hover:scale-110"/>
                    </a>

                    <a onclick={on_delete.clone()}>
                        <img src="/img/trash.svg" class="tournament-btn-icon hover:bg-red-400 cursor-pointer hover:scale-110"/>
                    </a>
                } else {
                    <span class="flex gap-0.5 pt-[5px]">
                        if *loading {
                            <LoadingCircle size={5} />
                        } else {
                            <span>{*nb_nuts}</span>
                        }
                        <img src="/img/nut.svg" class="tournament-btn-icon mt-[-5px]"/>
                    </span>

                    <a onclick={on_leave.clone()}>
                        <img src="/img/leave.svg" class="tournament-btn-icon hover:bg-red-400 cursor-pointer hover:scale-110"/>
                    </a>
                }
            </div>
        </div>
    }
}
