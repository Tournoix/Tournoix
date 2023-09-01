use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct FormInputProps {
    pub id: String,
    pub label: String,
    pub form_type: String,
    pub required: bool,
    #[prop_or_default]
    pub min_num: i32, // Only used for number
    #[prop_or_default]
    pub checked: bool, // Only used for checkbox
    pub disabled: Option<bool>,
    #[prop_or_default]
    pub _ref: NodeRef
}

#[function_component]
pub fn FormInput(props: &FormInputProps) -> Html {
    let FormInputProps { id, label, form_type, required, min_num, checked, disabled, _ref } = props;

    html! {
        <label class="flex flex-row-reverse m-4">
            <input required={required.clone()} class="ml-3 border bg-white drop-shadow" name={id.clone()} type={form_type.clone()} disabled={if let Some(disabled) = disabled {*disabled} else {false}} ref={_ref}
                min={min_num.to_string()}
                checked={checked.clone()}
            />
            <div>{label}</div>
        </label>
    }
}
