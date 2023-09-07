use dotenv_codegen::dotenv;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct JoinCodeProps {
    pub code: String,
}

#[function_component]
pub fn JoinCode(props: &JoinCodeProps) -> Html {
    let JoinCodeProps { code } = props;

    html! {
        <div class="flex items-center sm:flex-row flex-col">
            <span class="text-2xl mr-4">{"Code pour rejoindre ce tournoi :"}</span>
            <div class="flex flex-col items-center">
                <img src={format!("https://api.qrserver.com/v1/create-qr-code/?size=150x150&data={}/join/{}", dotenv!("APP_URL"), code)}/>
                <span class="text-lg">{ code }</span>
            </div>
        </div>
    }
}
