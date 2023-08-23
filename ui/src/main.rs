use yew::prelude::*;
use dotenv::dotenv;
use yew_router::{BrowserRouter, Switch};

use crate::routers::{Route, router};

mod pages;
mod routers;
mod layouts;

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