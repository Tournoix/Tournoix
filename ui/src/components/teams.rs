use time::Duration;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_notifications::use_notification;

use crate::{
    api::models::{AddTeamRequest, Team, Tournament},
    components::{loading_circle::LoadingCircle, team_card::TeamCard},
    notification::{CustomNotification, NotifType},
};

#[derive(PartialEq, Properties)]
pub struct TeamsProps {
    pub tournament: Tournament,
    #[prop_or_default]
    pub on_update: Callback<()>,
}

#[function_component]
pub fn Teams(props: &TeamsProps) -> Html {
    let TeamsProps {
        tournament,
        on_update,
    } = props;
    let notifs = use_notification::<CustomNotification>();

    let teams: UseStateHandle<Vec<Team>> = use_state(|| Vec::new());
    let teams_tmp: UseStateHandle<Vec<Team>> = use_state(|| Vec::new());

    let loading = use_state(|| true);
    let trigger = use_state(|| false);

    {
        let tournament = tournament.clone();
        let loading = loading.clone();
        let teams = teams.clone();
        let teams_tmp = teams_tmp.clone();
        let on_update = on_update.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    loading.set(true);
                    if let Some(t) = tournament.get_teams().await.ok() {
                        teams_tmp.set(t);
                        // Need to empty teams vec or else there is a weird behavior in rendering when adding/deleting team
                        teams.set(vec![]);
                    }
                    loading.set(false);
                });

                on_update.emit(());
            },
            trigger.clone(),
        );
    }

    // Use of a temp state to update teams vec
    // This way teams vec is first rendered as empty and then updated with values
    {
        let teams = teams.clone();

        use_effect_with_deps(
            move |teams_tmp| {
                teams.set((**teams_tmp).clone());
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

            let notifs = notifs.clone();
            spawn_local(async move {
                match tournament
                    .add_teams(AddTeamRequest {
                        name: "New team".into(),
                        group: 0,
                    })
                    .await
                {
                    Ok(new_team) => {
                        notifs.spawn(CustomNotification::new(
                            "Équipe ajoutée !",
                            &format!("L'équipe [{}] à été ajoutée", new_team.name),
                            NotifType::Success,
                            Duration::seconds(5),
                        ));

                        trigger.set(!*trigger);
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
        <div class="relative flex flex-col items-center bg-nutLighter p-3">
            if *loading {
                <div class="flex absolute top-0 left-0 justify-center items-center z-30 w-full h-full bg-black bg-opacity-25">
                    <LoadingCircle />
                </div>
            }
            <h3>{"Equipes ("}{teams.len()}{")"}</h3>
            <ul class="flex flex-wrap gap-3 justify-start items-center">
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
