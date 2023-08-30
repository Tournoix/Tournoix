use yew::prelude::*;

use crate::{layouts::homelayout::HomeLayout, components::backlink::Backlink, routers::Route};

#[derive(PartialEq, Properties)]
pub struct TournoixEditProps {
    pub id: i32,
}

#[function_component]
pub fn TournoixEdit(props: &TournoixEditProps) -> Html {
    let TournoixEditProps { id } = props;
    
    html! {
        <HomeLayout>
            <div class="flex flex-col items-center h-full pb-16 pt-12 sm:w-9/12 w-11/12 mx-auto relative">
                <Backlink route={Route::Tournoix} label="Retour à la liste des tournoix"/>
                <h1 class="mb-5">{"Modification de tournoi"}</h1>
                <h2>{"Id du tournoi : "}{ id }</h2>
            </div>
        </HomeLayout>
    }
}