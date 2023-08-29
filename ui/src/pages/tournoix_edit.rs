use yew::prelude::*;

use crate::layouts::homelayout::HomeLayout;

#[derive(PartialEq, Properties)]
pub struct TournoixEditProps {
    pub id: i32,
}

#[function_component]
pub fn TournoixEdit(props: &TournoixEditProps) -> Html {
    let TournoixEditProps { id } = props;
    
    html! {
        <HomeLayout>
            <div class="flex flex-col items-center h-full pb-16 sm:w-9/12 w-11/12 mx-auto relative">
                <h1 class="mt-12 mb-5">{"Modification de tournoi"}</h1>
                <h2>{"Id du tournoi : "}{ id }</h2>
            </div>
        </HomeLayout>
    }
}