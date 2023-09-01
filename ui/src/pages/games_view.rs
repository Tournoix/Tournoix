use yew::prelude::*;
use crate::{layouts::homelayout::HomeLayout, routers::Route, components::{backlink::Backlink, team_bet::TeamBet}};

#[derive(PartialEq, Properties)]
pub struct GamesViewProps {
    pub id: i32,
}

#[function_component]
pub fn BetView(props: &GamesViewProps) -> Html {
    let GamesViewProps { id } = props;
    
    html! {
        <HomeLayout>
            <div class="flex flex-col items-center h-full pb-16 pt-12 sm:w-9/12 w-11/12 mx-auto relative">
                <Backlink route={Route::TournoixView{ id: 42 }} label="Retour au tournoi"/>
                <h1 class="mb-5">{"Affichage du match"}</h1>
                <h2>{"Id du match : "}{ id }</h2>
                <TeamBet/>
            </div>
        </HomeLayout>
    }
}