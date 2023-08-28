use yew_router::prelude::*;
use yew::prelude::*;

use crate::pages::{home::Home, tournoix::Tournoix, login::Login, register::Register};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/tournoix")]
    Tournoix,
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
        Route::Login => html! {<Login/>},
        Route::Register => html! {<Register/>},
        Route::NotFound => html! {<h1>{"404 Not Found"}</h1>}
    }
}