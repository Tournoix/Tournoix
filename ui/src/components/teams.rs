use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{
    api::models::{AddTeamRequest, Team, Tournament},
    components::{loading_circle::LoadingCircle, notification::NotifType, team_card::TeamCard},
    utils::utils::add_delayed_notif,
};

#[derive(PartialEq, Properties)]
pub struct TeamsProps {
    pub tournament: Tournament,
    /*
    pub on_create: Option<Callback<MouseEvent>>,
    pub on_edit: Option<Callback<i32>>,
    pub on_delete: Option<Callback<i32>>,
    */
}

#[function_component]
pub fn Teams(props: &TeamsProps) -> Html {
    let TeamsProps { tournament } = props;

    let teams: UseStateHandle<Vec<Team>> = use_state(|| Vec::new());
    let teams_tmp: UseStateHandle<Vec<Team>> = use_state(|| Vec::new());

    let loading = use_state(|| true);
    let trigger = use_state(|| false);

    {
        let tournament = tournament.clone();
        let loading = loading.clone();
        let teams = teams.clone();
        let teams_tmp = teams_tmp.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    loading.set(true);
                    if let Some(t) = tournament.get_teams().await.ok() {
                        teams_tmp.set(t);
                        // Need to empty teams vec or else there is a weird behavior in rendering when adding/deleting team
                        teams.set(vec![]); 
                    }
                });

                || ()
            },
            trigger.clone(),
        );
    }

    // Use of a temp state to update teams vec
    // This way teams vec is first rendered as empty and then updated with values
    {
        let teams = teams.clone();
        let loading = loading.clone();

        use_effect_with_deps(
            move |teams_tmp| {
                teams.set((**teams_tmp).clone());
                loading.set(false);
            },
            teams_tmp,
        );
    }

    let on_create = {
        let tournament = tournament.clone();
        let trigger = trigger.clone();

        Callback::from(move |_| {
            let tournament = tournament.clone();
            let trigger = trigger.clone();

            spawn_local(async move {
                match tournament
                    .add_teams(AddTeamRequest {
                        name: "New team".into(),
                        group: 0,
                    })
                    .await
                {
                    Ok(new_team) => {
                        add_delayed_notif(
                            "Équipe ajoutée !",
                            &format!("L'équipe [{}] à été ajoutée", new_team.name),
                            NotifType::Success,
                        );

                        trigger.set(!*trigger);
                    }

                    Err(e) => {
                        add_delayed_notif(
                            &format!("Erreur: {}", e.error.reason),
                            &e.error.description,
                            NotifType::Error,
                        );
                    }
                }
            });
        })
    };

    html! {
        <div class="relative flex flex-col items-center bg-nutLighter p-3">
            if *loading {
                <div class="flex absolute top-0 left-0 justify-center items-center z-30 w-full h-full bg-black bg-opacity-25">
                    <LoadingCircle />
                </div>
            }
            <h3>{"Equipes"}</h3>
            <ul class="flex flex-wrap gap-3 justify-center items-center">
                <li class="team-item team-selectable">
                    <div class="team-name rounded text-center" onclick={on_create}>
                        <img src="/img/plus.svg" class="add-btn"/>
                        {"Créer une équipe"}
                    </div>
                </li>
                {
                    teams.iter().map(|team| {
                        html!{
                            <TeamCard team={team.clone()} update_trigger={trigger.clone()} />
                        }
                    }).collect::<Html>()
                }
            </ul>
        </div>
    }
}
