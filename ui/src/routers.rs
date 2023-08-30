use yew_router::prelude::*;
use yew::prelude::*;

use crate::pages::{home::Home, tournoix::Tournoix, login::Login, register::Register, tournoix_edit::TournoixEdit, tournoix_view::TournoixView, tournoix_create::TournoixCreate};

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
        Route::Tournoix => html! {<Tournoix/>},
        Route::TournoixView { id } => html! {<TournoixView id={id} />},
        Route::TournoixEdit { id }=> html! {<TournoixEdit id={id} />},
        Route::TournoixCreate => html! {<TournoixCreate />},
        Route::Login => html! {<Login/>},
        Route::Register => html! {<Register/>},
        Route::NotFound => html! {<h1>{"404 Not Found"}</h1>}
    }
}