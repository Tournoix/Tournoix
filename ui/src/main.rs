use dotenv::dotenv;
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

use crate::{
    components::user_provider::UserProvider,
    routers::{router, Route},
};

mod api;
mod components;
mod layouts;
mod pages;
mod routers;
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
