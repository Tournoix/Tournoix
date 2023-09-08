use time::Duration;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_notifications::use_notification;

use crate::{
    api::models::{Team, TeamUpdate},
    notification::{CustomNotification, NotifType}, utils::utils::team_color_wrapper,
};

#[derive(PartialEq, Properties)]
pub struct TeamCardProps {
    pub team: Team,
    pub update_trigger: UseStateHandle<bool>,
}

#[function_component]
pub fn TeamCard(props: &TeamCardProps) -> Html {
    let TeamCardProps {
        team,
        update_trigger,
    } = props;

    let team = use_state(|| team.clone());
    let is_being_edited = use_state(|| false);
    let name_ref = use_node_ref();
    let notifs = use_notification::<CustomNotification>();

    let on_edit = {
        let is_being_edited = is_being_edited.clone();
        let name_ref = name_ref.clone();
        let team = team.clone();
        let update_trigger = update_trigger.clone();
        let notifs = notifs.clone();

        Callback::from(move |_| {
            let team = team.clone();
            let update_trigger = update_trigger.clone();

            if *is_being_edited {
                let new_name = name_ref.cast::<HtmlInputElement>().unwrap().value();
                let mut new_team = (*team).clone();
                new_team.name = new_name.clone();
                team.set(new_team);

                let notifs = notifs.clone();
                spawn_local(async move {
                    match team
                        .update(TeamUpdate {
                            name: Some(new_name),
                            group: None,
                        })
                        .await
                    {
                        Ok(team) => {
                            notifs.spawn(CustomNotification::new(
                                "Équipe modifiée !",
                                &format!("L'équipe [{}] à été modifiée", team.name),
                                NotifType::Success,
                                Duration::seconds(5),
                            ));

                            update_trigger.set(!*update_trigger);
                        }

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

                is_being_edited.set(false);
            } else {
                is_being_edited.set(true);
            }
        })
    };

    {
        let name_ref = name_ref.clone();

        use_effect_with_deps(
            move |_| {
                name_ref
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .focus()
                    .unwrap();
            },
            is_being_edited.clone(),
        );
    }

    let on_delete = {
        let update_trigger = update_trigger.clone();
        let team = team.clone();

        Callback::from(move |_| {
            let team = team.clone();
            let update_trigger = update_trigger.clone();

            let notifs = notifs.clone();
            spawn_local(async move {
                match team.delete().await {
                    Ok(_) => {
                        notifs.spawn(CustomNotification::new(
                            "Équipe supprimée !",
                            &format!("L'équipe [{}] à été supprimée", team.name),
                            NotifType::Success,
                            Duration::seconds(5),
                        ));

                        update_trigger.set(!*update_trigger);
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
        <div class="team-item">
            <div style={team_color_wrapper(team.name.clone())} class={"team-name rounded-t border-b-4 team-border-color"}>
                <input class={format!("w-full text-center {}", if *is_being_edited { "bg-yellow-200" } else { "bg-transparent" })} disabled={!*is_being_edited} type="text" value={team.name.clone()} ref={name_ref} />
            </div>
            <div class="team-btn-list">
                <a onclick={on_edit}>
                    <img src={if *is_being_edited { "/img/checkmark.svg" } else { "/img/pencil.svg" }} class={format!("team-btn-icon cursor-pointer hover:scale-110 {}", {if *is_being_edited { "hover:bg-green-400" } else { "hover:bg-orange-400" }})}/>
                </a>

                <a onclick={on_delete}>
                    <img src="/img/trash.svg" class="team-btn-icon hover:bg-red-400 cursor-pointer hover:scale-110"/>
                </a>
            </div>
        </div>
    }
}
