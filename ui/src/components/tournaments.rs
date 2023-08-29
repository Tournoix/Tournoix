use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct TournamentsProps {
    pub tournaments: Vec<String>,
    pub on_create: Option<Callback<MouseEvent>>,
    pub on_read: Callback<MouseEvent>,
    pub on_edit: Option<Callback<MouseEvent>>,
    pub on_delete: Option<Callback<MouseEvent>>,
    pub nb_nuts: Option<i32>,
    pub on_leave: Option<Callback<MouseEvent>>,
}
// TODO when invoking a on_... function, pass the id of the tournament to it

#[function_component]
pub fn Tournaments(props: &TournamentsProps) -> Html {
    let TournamentsProps { tournaments, on_create, on_read, on_delete, on_edit, nb_nuts, on_leave } = props;

    html! {
        <ul class="flex gap-5 flex-wrap">
            // Create a tournament
            {if let Some(_on_create) = on_create {
                html! { <li class="tournament-card flex-col text-center" onclick={_on_create}>
                    <img src="/img/plus.svg" class="h-1/2 mb-1"/>
                    {"Créer un tournoi"}
                </li> }
            } else { html! {}}}
            
            // List tournaments
            {
                tournaments.into_iter().map(|tournament| {
                    html!{<li>
                        <div class="tournament-card" onclick={on_read}>{ tournament }</div>
                        <div class="tournament-btn-list">

                            // Edit
                            {if let Some(_on_edit) = on_edit {
                                html! { <a onclick={_on_edit}>
                                    <img src="/img/pencil.svg" class="tournament-btn-icon hover:bg-orange-400 cursor-pointer hover:scale-110"/>
                                </a> }
                            } else { html! {}}}

                            // Delete
                            {if let Some(_on_delete) = on_delete {
                                html! { <a onclick={_on_delete}>
                                    <img src="/img/trash.svg" class="tournament-btn-icon hover:bg-red-400 cursor-pointer hover:scale-110"/>
                                </a> }
                            } else { html! {}}}

                            // Nut number
                            {if let Some(_nb_nuts) = nb_nuts {
                                html! { <span class="flex gap-1">
                                    <span>{_nb_nuts}</span><img src="/img/nut.svg" class="tournament-btn-icon"/>
                                </span> }
                            } else { html! {}}}

                            // Leave
                            {if let Some(_on_leave) = on_leave {
                                html! { <a onclick={_on_leave}>
                                    <img src="/img/leave.svg" class="tournament-btn-icon hover:bg-red-400 cursor-pointer hover:scale-110"/>
                                </a> }
                            } else { html! {}}}
                        </div>
                    </li>}
                }).collect::<Html>()
            }
        </ul>
    }
}