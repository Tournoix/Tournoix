use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CheckBoxProps {
    #[prop_or_default]
    pub class: Classes,
    pub id: String,
    pub label: String,
    pub disabled: Option<bool>,
    pub checked: bool,
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub _ref: NodeRef
}

#[function_component]
pub fn CheckBox(props: &CheckBoxProps) -> Html {
    let CheckBoxProps { class, id, label, disabled, checked, on_click, _ref } = props;

    html! {
        <label class={classes!(class.clone(), "flex", "flex-row-reverse", "m-4")}>
            <input class="ml-3 border bg-white drop-shadow" name={id.clone()} type="checkbox" checked={checked.clone()} onclick={on_click} disabled={if let Some(disabled) = disabled {*disabled} else {false}} ref={_ref}/>
            <div>{label}</div>
        </label>
    }
}
