use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct Team {
    pub id: i32,
    pub is_being_edited: bool,
    pub name: String,
}

#[derive(PartialEq, Properties)]
pub struct TeamsProps {
    pub on_create: Option<Callback<MouseEvent>>,
    pub on_edit: Option<Callback<i32>>,
    pub on_delete: Option<Callback<i32>>,
}

#[function_component]
pub fn Teams(props: &TeamsProps) -> Html {
    let TeamsProps { on_create, on_edit, on_delete } = props;

    let teams = use_context::<UseStateHandle<Vec<Team>>>().expect("Missing teams provider");

    let on_edit_click = |id: i32| {
        if let Some(on_edit) = on_edit {
            let on_edit = on_edit.clone();

            Callback::from(move |_| {
                on_edit.emit(id);
                ()
            })
        } else {
            Callback::noop()
        }
    };

    let on_delete_click = |id: i32| {
        if let Some(on_delete) = on_delete {
            let on_delete = on_delete.clone();

            Callback::from(move |_| {
                on_delete.emit(id);
                ()
            })
        } else {
            Callback::noop()
        }
    };

    html! {
        <div class="flex flex-col items-center bg-nutLighter p-3">
            <h3>{"Equipes"}</h3>
            <ul class="flex flex-wrap gap-3 justify-center items-center">
                {if let Some(on_create) = on_create {
                    html! { <li class="team-item team-selectable">
                        <div class="team-name rounded text-center" onclick={on_create}>
                            <img src="/img/plus.svg" class="add-btn"/>
                            {"Créer une équipe"}
                        </div>
                    </li>}
                } else { html! {}}}
                {
                    teams.iter().map(|team| {
                        html!{<li class="team-item">
                            <div class={format!("team-name {}", if let Some(_on_edit) = on_edit { "rounded-t" } else { "rounded" })}>
                                <input id={format!("input-team-{}", team.id)} class={format!("w-full text-center {}", if team.is_being_edited { "bg-yellow-200" } else { "bg-transparent" })} disabled={!team.is_being_edited} type="text" value={team.name.clone()}/>
                            </div>
                            if let Some(_on_edit) = on_edit {
                                <div class="team-btn-list">
                                    // Edit
                                    {if let Some(_on_edit) = on_edit {
                                        html! { <a onclick={on_edit_click(team.id)}>
                                            <img src={if team.is_being_edited { "/img/checkmark.svg" } else { "/img/pencil.svg" }} class={format!("team-btn-icon cursor-pointer hover:scale-110 {}", {if team.is_being_edited { "hover:bg-green-400" } else { "hover:bg-orange-400" }})}/>
                                        </a> }
                                    } else { html! {}}}
        
                                    // Delete
                                    {if let Some(on_delete) = on_delete {
                                        html! { <a onclick={on_delete_click(team.id)}>
                                            <img src="/img/trash.svg" class="team-btn-icon hover:bg-red-400 cursor-pointer hover:scale-110"/>
                                        </a> }
                                    } else { html! {}}}
                                </div>
                            }
                        </li>}
                    }).collect::<Html>()
                }
            </ul>
        </div>
    }
}