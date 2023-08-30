use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};
use dotenv::dotenv;

use crate::routers::{Route, router};

mod pages;
mod routers;
mod layouts;
mod components;
mod utils;

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={router} />
        </BrowserRouter>
    }
}

fn main() {
    dotenv().ok();

    yew::Renderer::<App>::new().render();
}