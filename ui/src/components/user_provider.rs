use std::rc::Rc;
use web_sys::window;
use yew::prelude::*;
use yew_hooks::use_effect_once;

#[derive(Clone, Debug, PartialEq)]
pub struct UserInfo {
    reducer: Option<UseReducerHandle<UserInfo>>
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

    pub fn token(&self) ->  Option<String> {
        UserInfo::get_token()
    }

    pub fn login(&self, token: &str) -> bool {
        if let Some(win) = window() {
            if let Ok(Some(store)) = win.local_storage() {
                return store.set_item("loginToken", token).is_ok();
            }
        }

        false
    }

    pub fn logout(&self) -> bool {
        if let Some(win) = window() {
            if let Ok(Some(store)) = win.local_storage() {
                if store.remove_item("loginToken").is_ok() {
                    self.reducer.as_ref().unwrap().dispatch((Action::Refresh, None));
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
    SetReducer
}

impl Reducible for UserInfo {
    type Action = (Action, Option<UseReducerHandle<UserInfo>>);

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_data = (*self).clone();

        match action.0 {
            Action::Refresh => {
                window().unwrap().location().reload().unwrap();
            },
            Action::SetReducer => {
                new_data.reducer = action.1
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
    let UserProviderProps {children} = props;

    let user_reducer = use_reducer(|| UserInfo {reducer: None});
    
    {
        let user_reducer = user_reducer.clone();

        use_effect_once(move || {
            user_reducer.dispatch((Action::SetReducer, Some(user_reducer.clone())));
    
            || ()
        });
    }

    html! {
        <ContextProvider<UserContext> context={user_reducer}>
            {for children.iter()}
        </ContextProvider<UserContext>>
    }
}