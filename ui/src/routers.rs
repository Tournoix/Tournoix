use yew_router::prelude::*;
use yew::prelude::*;

use crate::{pages::{home::Home, tournoix::Tournoix, login::Login, register::Register, tournoix_edit::TournoixEdit, tournoix_view::TournoixView, games_view::BetView, tournoix_create::TournoixCreate}, components::user_provider::UserContext};

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
    #[at("/tournoix/bet/:id")]
    BetView { id: i32 },
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
        Route::TournoixEdit { id }=> html! {<LoggedRoute><TournoixEdit id={id} /></LoggedRoute>},
        Route::TournoixCreate => html! {<LoggedRoute><TournoixCreate /></LoggedRoute>},
        Route::BetView { id }=> html! {<LoggedRoute><BetView id={id} /></LoggedRoute>},
        Route::Login => html! {<Login/>},
        Route::Register => html! {<Register/>},
        Route::NotFound => html! {<h1>{"404 Not Found"}</h1>}
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

    html ! {
        if user_info.is_logged() {
            {children.clone()}
        } else {
            <Redirect<Route> to={Route::Login} />
        }
    }
}