use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    components::user_provider::UserContext,
    pages::{
        home::Home, login::Login, register::Register, tournoix::Tournoix,
        tournoix_create::TournoixCreate, tournoix_edit::TournoixEdit, tournoix_view::TournoixView, games_view::MatchView, join::Join,
    },
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/tournoix")]
    Tournoix,
    #[at("/tournoix/create")]
    TournoixCreate,
    #[at("/tournoix/:id/edit")]
    TournoixEdit { id: i32 },
    #[at("/tournoix/:id")]
    TournoixView { id: i32 },
    #[at("/tournoix/:tournament_id/match/:match_id")]
    MatchView { tournament_id: i32, match_id: i32 },
    #[at("/join/:code")]
    Join {code: String},
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn router(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home/> },
        Route::Tournoix => html! {<LoggedRoute><Tournoix/></LoggedRoute>},
        Route::TournoixView { id } => html! {<LoggedRoute><TournoixView id={id} /></LoggedRoute>},
        Route::TournoixEdit { id } => html! {<LoggedRoute><TournoixEdit id={id} /></LoggedRoute>},
        Route::TournoixCreate => html! {<LoggedRoute><TournoixCreate /></LoggedRoute>},
        Route::MatchView { tournament_id, match_id } => html! {<LoggedRoute><MatchView tournament_id={tournament_id} match_id={match_id} /></LoggedRoute>},
        Route::Join { code } => html! {<LoggedRoute><Join code={code}/></LoggedRoute>},
        Route::Login => html! {<Login/>},
        Route::Register => html! {<Register/>},
        Route::NotFound => html! {<h1>{"404 Not Found"}</h1>},
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct LoggedRouteProps {
    pub children: Children,
}

#[function_component]
pub fn LoggedRoute(props: &LoggedRouteProps) -> Html {
    let LoggedRouteProps { children } = props;
    let user_info = use_context::<UserContext>().expect("Missing user context provider");

    html! {
        if user_info.is_logged() {
            {children.clone()}
        } else {
            <Redirect<Route> to={Route::Login} />
        }
    }
}
