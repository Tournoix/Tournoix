use yew::prelude::*;

use crate::layouts::homelayout::HomeLayout;
use crate::components::tournaments::Tournaments;

#[derive(PartialEq, Properties)]
pub struct TournoixProps {}

#[function_component]
pub fn Tournoix(props: &TournoixProps) -> Html {
    let TournoixProps {} = props;

    let on_delete_click = Callback::from(move |_| { });
    let on_edit_click = Callback::from(move |_| { });
    let on_leave_click = Callback::from(move |_| { });
    
    html! {
        <HomeLayout>
            <div class="flex flex-col items-center h-full pb-16 sm:w-9/12 w-11/12 mx-auto relative">
                <h1 class="mt-12 mb-5">{"Liste des tournoix"}</h1>
                <h2 class="mt-12 mb-5">{"Mes tournoix"}</h2>
                <Tournaments on_delete={on_delete_click} on_edit={on_edit_click}/>
                <h2 class="mt-12 mb-5">{"Tournoix rejoins"}</h2>
                <Tournaments nb_nuts={42} on_leave={on_leave_click}/>
            </div>
        </HomeLayout>
    }
}