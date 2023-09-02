use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct Group; // TODO add fields ?

#[derive(PartialEq, Properties)]
pub struct GroupsProps {
    pub on_create: Option<Callback<MouseEvent>>,
    pub on_delete: Option<Callback<usize>>,
}

#[function_component]
pub fn Groups(props: &GroupsProps) -> Html {
    let GroupsProps { on_create, on_delete } = props;

    let groups = use_context::<UseStateHandle<Vec<Group>>>().expect("Missing groups provider");

    let on_delete_click = |id: usize| {
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
                        html!{<li class="group-item group-selectable" onclick={on_create}>
                            <img src="/img/plus.svg" class="add-btn"/>
                            {"Créer un groupe"}
                        </li>}
                    } else { html! {}}
                }
                {
                    groups.iter().enumerate().map(|(index, _group)| {
                        html!{<li class="group-item relative">
                            <div class="flex justify-center">
                                <div>{"Groupe "}{index + 1}</div>
                                if let Some(_on_delete) = on_delete {
                                    <div class="ml-2">
                                        <img onclick={on_delete_click(index)} src="/img/trash.svg" class="group-btn-icon absolute top-0 right-0 hover:bg-red-400 bg-white cursor-pointer hover:scale-110"/>
                                    </div>
                                }
                            </div>
                        </li>}
                    }).collect::<Html>()
                }
            </ul>
        </div>
    }
}