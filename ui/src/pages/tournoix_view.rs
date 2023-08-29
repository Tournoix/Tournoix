use yew::prelude::*;

use crate::layouts::homelayout::HomeLayout;

#[derive(PartialEq, Properties)]
pub struct TournoixViewProps {
    pub id: i32,
}

#[function_component]
pub fn TournoixView(props: &TournoixViewProps) -> Html {
    let TournoixViewProps { id } = props;
    
    html! {
        <HomeLayout>
            <div class="flex flex-col items-center h-full pb-16 sm:w-9/12 w-11/12 mx-auto relative">
                <h1 class="mt-12 mb-5">{"Affichage de tournoi"}</h1>
                <h2>{"Id du tournoi : "}{ id }</h2>
            </div>
        </HomeLayout>
    }
}