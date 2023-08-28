use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct FormInputProps {
    pub id: String,
    pub label: String,
    pub form_type: String,
    pub required: bool,
}

#[function_component]
pub fn FormInput(props: &FormInputProps) -> Html {
    let FormInputProps { id, label, form_type, required } = props;

    html! {
        <label class="flex flex-row-reverse m-4">
            <input required={required.clone()} class="ml-3 border bg-white drop-shadow" name={id.clone()} type={form_type.clone()}/>
            <div>{label}</div>
        </label>
    }
}
