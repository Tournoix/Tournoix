use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct Group {
    pub id: i32,
    pub name: String,
}

#[derive(PartialEq, Properties)]
pub struct GroupsProps {
    pub on_create: Option<Callback<MouseEvent>>,
    pub on_delete: Option<Callback<i32>>,
}

#[function_component]
pub fn Groups(props: &GroupsProps) -> Html {
    let GroupsProps { on_create, on_delete } = props;

    let groups = use_context::<UseStateHandle<Vec<Group>>>().expect("Missing groups provider");

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
        <div class="flex flex-col items-center p-3">
            <h3>{"Groupes"}</h3>
            <ul class="flex flex-wrap gap-3">
                {
                    if let Some(on_create) = on_create {
                        html!{<li class="group-item group-selectable">
                            <img src="/img/plus.svg" class="add-btn"/>
                            {"Créer un groupe"}
                        </li>}
                    } else { html! {}}
                }
                {
                    groups.iter().map(|group| {
                        html!{<li class="group-item">
                            <div>
                                {"Groupe "}{group.name.clone()}
                            </div>
                            if let Some(on_delete) = on_delete {
                                <div class="group-btn-list">
                                    // Delete
                                    <a onclick={on_delete_click(group.id)}>
                                        <img src="/img/trash.svg" class="group-btn-icon hover:bg-red-400 cursor-pointer hover:scale-110"/>
                                    </a>
                                </div>
                            }
                        </li>}
                    }).collect::<Html>()
                }
            </ul>
        </div>
    }
}