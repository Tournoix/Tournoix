use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct FormInputProps {
    pub id: String,
    pub label: String,
    pub form_type: String,
    pub required: bool,
    pub min_num: Option<i32>,
    #[prop_or_default]
    pub _ref: NodeRef
}

#[function_component]
pub fn FormInput(props: &FormInputProps) -> Html {
    let FormInputProps { id, label, form_type, required, min_num, _ref } = props;

    html! {
        <label class="flex flex-row-reverse m-4">
            <input required={required.clone()} class="ml-3 border bg-white drop-shadow" name={id.clone()} type={form_type.clone()} ref={_ref}
                min={if let Some(min_num) = min_num.clone() { min_num.to_string() } else { "".to_string() }}
            />
            <div>{label}</div>
        </label>
    }
}
