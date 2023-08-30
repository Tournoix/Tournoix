use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::layouts::homelayout::HomeLayout;
use crate::components::tournaments::Tournaments;
use crate::routers::Route;

#[derive(PartialEq, Properties)]
pub struct TournoixProps {}

#[function_component]
pub fn Tournoix(props: &TournoixProps) -> Html {
    let TournoixProps {} = props;
    let navigator = use_navigator().unwrap();

    let on_create_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::TournoixCreate))
    };
    let on_read_click = Callback::from(move |_| { });
    let on_edit_click = Callback::from(move |_| { });
    let on_delete_click = Callback::from(move |_| { });
    let on_leave_click = Callback::from(move |_| { });

    // Test data
    let mut owned_tournaments: Vec<String> = Vec::new();
    owned_tournaments.push("LAN Leco 2023".to_string());
    owned_tournaments.push("Pétanque FVJC".to_string());

    let mut joined_tournaments: Vec<String> = Vec::new();
    joined_tournaments.push("Z-event 2022".to_string());
    joined_tournaments.push("Z-event 2023".to_string());
    
    html! {
        <HomeLayout>
            <div class="flex flex-col items-center h-full pb-16 sm:w-9/12 w-11/12 mx-auto relative">
                <h1 class="mt-12 mb-5">{"Liste des tournoix"}</h1>
                <h2 class="mt-12 mb-5">{"Mes tournoix"}</h2>
                <Tournaments tournaments={owned_tournaments} on_read={on_read_click.clone()} on_create={on_create_click} on_delete={on_delete_click} on_edit={on_edit_click}/>
                <h2 class="mt-12 mb-5">{"Tournoix rejoins"}</h2>
                <Tournaments tournaments={joined_tournaments} on_read={on_read_click} nb_nuts={42} on_leave={on_leave_click}/>
            </div>
        </HomeLayout>
    }
}