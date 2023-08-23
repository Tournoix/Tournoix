use yew_router::prelude::*;
use yew::prelude::*;

use crate::pages::{home::Home, tournoix::Tournoix};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/tournoix")]
    Tournoix,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn router(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home/> },
        Route::Tournoix => html! {<Tournoix/>},
        Route::NotFound => html! {<h1>{"404 Not Found"}</h1>}
    }
}