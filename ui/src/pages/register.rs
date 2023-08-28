use yew::prelude::*;

use crate::components::{form_input::FormInput, button::Button};
use crate::layouts::homelayout::HomeLayout;

#[derive(PartialEq, Properties)]
pub struct RegisterProps {}

#[function_component]
pub fn Register(props: &RegisterProps) -> Html {
    let RegisterProps {} = props;

    let on_register_click: Callback<MouseEvent> = Callback::from(|_| {
        
    });

    html! {
        <HomeLayout>
            <div class="flex flex-col items-center h-full sm:w-9/12 w-11/12 mx-auto relative">
                <h1 class="mt-12 mb-5">{"Créer un compte"}</h1>
                <form class="flex flex-col sm:text-xl text-lg" id="login-form">
                    <FormInput id="email" label="E-mail" form_type="email" required={true}/>
                    <FormInput id="username" label="Nom d'utilisateur" form_type="text" required={true}/>
                    <FormInput id="password" label="Mot de passe" form_type="password" required={true}/>
                    <Button class="px-3 py-2 mx-auto mt-3 mb-16 hover:scale-110" onclick={on_register_click}>{"Créer un compte"}</Button>
                </form>
            </div>
        </HomeLayout>
    }
}
