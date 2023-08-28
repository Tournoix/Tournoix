use yew::prelude::*;

use crate::layouts::homelayout::HomeLayout;
use crate::components::tournaments::Tournaments;

#[derive(PartialEq, Properties)]
pub struct TournoixProps {}

#[function_component]
pub fn Tournoix(props: &TournoixProps) -> Html {
    let TournoixProps {} = props;
    
    html! {
        <HomeLayout>
            <div class="flex flex-col items-center h-full pb-16 sm:w-9/12 w-11/12 mx-auto relative">
                <h1 class="mt-12 mb-5">{"Liste des tournoix"}</h1>
                <h2 class="mt-12 mb-5">{"Mes tournoix"}</h2>
                <Tournaments/>
                <h2 class="mt-12 mb-5">{"Tournoix rejoins"}</h2>
                <Tournaments/>
            </div>
        </HomeLayout>
    }
}