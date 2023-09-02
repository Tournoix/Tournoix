use std::rc::Rc;
use web_sys::window;
use yew::{platform::spawn_local, prelude::*};
use yew_hooks::use_effect_once;

use crate::api::{self, models::User};

#[derive(Clone, Debug, PartialEq)]
pub struct UserInfo {
    pub user: Option<User>,
    reducer: Option<UseReducerHandle<UserInfo>>,
}

impl UserInfo {
    pub fn get_token() -> Option<String> {
        if let Some(win) = window() {
            if let Ok(Some(store)) = win.local_storage() {
                if let Ok(item) = store.get_item("loginToken") {
                    return item;
                }
            }
        }

        None
    }

    pub fn token(&self) -> Option<String> {
        UserInfo::get_token()
    }

    pub fn login(&self, token: &str) -> bool {
        if let Some(win) = window() {
            if let Ok(Some(store)) = win.local_storage() {
                let _ = store.set_item("loginToken", token);
                let reducer = self.reducer.clone();

                spawn_local(async move {
                    let user = api::me().await.ok();
                    reducer
                        .as_ref()
                        .unwrap()
                        .dispatch((Action::SetUser, None, user));
                });
            }
        }

        false
    }

    pub fn logout(&self) -> bool {
        if let Some(win) = window() {
            if let Ok(Some(store)) = win.local_storage() {
                if store.remove_item("loginToken").is_ok() {
                    self.reducer
                        .as_ref()
                        .unwrap()
                        .dispatch((Action::Refresh, None, None));
                    return true;
                }
            }
        }

        false
    }

    pub fn is_logged(&self) -> bool {
        self.token().is_some()
    }
}

pub enum Action {
    Refresh,
    SetReducer,
    SetUser,
}

impl Reducible for UserInfo {
    type Action = (Action, Option<UseReducerHandle<UserInfo>>, Option<User>);

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_data = (*self).clone();

        match action.0 {
            Action::Refresh => {
                window().unwrap().location().reload().unwrap();
            }
            Action::SetReducer => {
                new_data.reducer = action.1;
            }
            Action::SetUser => {
                new_data.user = action.2;
            }
        };

        new_data.into()
    }
}

#[derive(PartialEq, Properties)]
pub struct UserProviderProps {
    pub children: Children,
}

pub type UserContext = UseReducerHandle<UserInfo>;

#[function_component]
pub fn UserProvider(props: &UserProviderProps) -> Html {
    let UserProviderProps { children } = props;

    let user_reducer = use_reducer(|| UserInfo {
        user: None,
        reducer: None,
    });

    {
        let user_reducer = user_reducer.clone();

        use_effect_once(move || {
            user_reducer.dispatch((Action::SetReducer, Some(user_reducer.clone()), None));
            //executor::block_one();

            spawn_local(async move {
                let user = api::me().await.ok();
                user_reducer.dispatch((Action::SetUser, None, user));
            });

            || ()
        });
    }

    html! {
        <ContextProvider<UserContext> context={user_reducer}>
            {for children.iter()}
        </ContextProvider<UserContext>>
    }
}
