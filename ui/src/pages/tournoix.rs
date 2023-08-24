use yew::prelude::*;

use crate::layouts::homelayout::HomeLayout;

#[derive(PartialEq, Properties)]
pub struct TournoixProps {}

#[function_component]
pub fn Tournoix(props: &TournoixProps) -> Html {
    let TournoixProps {} = props;
    
    html! {
        <HomeLayout>
            <div class={"flex flex-col p-4"}>
                {"Come back later"}
            </div>
        </HomeLayout>
    }
}