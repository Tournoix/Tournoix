use dotenv::dotenv;
use yew::prelude::*;
use yew_notifications::{NotificationsPosition, NotificationsProvider};
use yew_router::{BrowserRouter, Switch};

use crate::{
    components::user_provider::UserProvider,
    notification::{factory::CustomNotificationFactory, CustomNotification},
    routers::{router, Route},
};

mod api;
mod components;
mod layouts;
mod notification;
mod pages;
mod routers;
mod utils;

#[function_component]
fn App() -> Html {
    let component_creator = CustomNotificationFactory::default();

    html! {
        <NotificationsProvider<CustomNotification, CustomNotificationFactory> {component_creator} position={NotificationsPosition::BottomRight}>
            <UserProvider>
                <BrowserRouter>
                    <Switch<Route> render={router} />
                </BrowserRouter>
            </UserProvider>
        </NotificationsProvider<CustomNotification, CustomNotificationFactory>>
    }
}

fn main() {
    dotenv().ok();

    let _ = console_log::init_with_level(log::Level::Debug);

    yew::Renderer::<App>::new().render();
}
