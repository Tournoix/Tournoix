use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct TournamentsProps {
    pub on_delete: Option<Callback<MouseEvent>>,
    pub on_edit: Option<Callback<MouseEvent>>,
    pub nb_nuts: Option<i32>,
    pub on_leave: Option<Callback<MouseEvent>>,
}

#[function_component]
pub fn Tournaments(props: &TournamentsProps) -> Html {
    let TournamentsProps { on_delete, on_edit, nb_nuts, on_leave } = props;
    let names = vec!["Z-event", "LAN Leco 2023", "Pétanque FVJC"];

    html! {
        <ul class="flex gap-5 flex-wrap">
            <li class="tournament-card">{"Ajouter un tournoi"}</li>
            {
                names.into_iter().map(|name| {
                    html!{<li>
                        <div class="tournament-card" key={name}>{ name }</div>
                        <div class="tournament-btn-list">

                            {if let Some(_on_edit) = on_edit {
                                html! { <a onclick={_on_edit}>
                                    <img src="/img/pencil.svg" class="tournament-btn-icon hover:bg-orange-400 cursor-pointer hover:scale-110"/>
                                </a> }
                            } else { html! {}}}

                            {if let Some(_on_delete) = on_delete {
                                html! { <a onclick={_on_delete}>
                                    <img src="/img/trash.svg" class="tournament-btn-icon hover:bg-red-400 cursor-pointer hover:scale-110"/>
                                </a> }
                            } else { html! {}}}

                            {if let Some(_nb_nuts) = nb_nuts {
                                html! { <span class="flex gap-1">
                                    <span>{_nb_nuts}</span><img src="/img/nut.svg" class="tournament-btn-icon"/>
                                </span> }
                            } else { html! {}}}

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