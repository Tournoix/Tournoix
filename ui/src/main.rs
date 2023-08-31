use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};
use dotenv::dotenv;

use crate::{routers::{Route, router}, components::user_provider::UserProvider};

mod pages;
mod routers;
mod layouts;
mod components;
mod utils;

#[function_component]
fn App() -> Html {
    html! {
        <UserProvider>
            <BrowserRouter>
                <Switch<Route> render={router} />
            </BrowserRouter>
        </UserProvider>
    }
}

fn main() {
    dotenv().ok();

    console_log::init_with_level(log::Level::Debug);

    yew::Renderer::<App>::new().render();
}