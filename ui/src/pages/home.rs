use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{routers::Route, layouts::homelayout::HomeLayout};

#[derive(PartialEq, Properties)]
pub struct HomeProps {}

#[function_component]
pub fn Home(props: &HomeProps) -> Html {
    let HomeProps {} = props;
    let navigator = use_navigator().unwrap();
    
    let onclick = Callback::from(move |_| navigator.push(&Route::Tournoix));

    html! {
        <HomeLayout>
            <div class={"flex flex-col"}>
                {"Welcome home"}
                <button {onclick}>{"Go to Tournoix"}</button>
            </div>
        </HomeLayout>
    }
}