use yew::prelude::*;

use crate::{layouts::homelayout::HomeLayout, components::{form_input::FormInput, button::Button}};

#[derive(PartialEq, Properties)]
pub struct TournoixCreateProps {
}

#[function_component]
pub fn TournoixCreate(props: &TournoixCreateProps) -> Html {
    let TournoixCreateProps {} = props;

    let on_create_click = Callback::from(move |_| { });
    
    html! {
        <HomeLayout>
            <div class="flex flex-col items-center h-full pb-16 sm:w-9/12 w-11/12 mx-auto relative">
                <h1 class="mt-12 mb-5">{"Création de tournoi"}</h1>
                <h2>{"Général"}</h2>
                <form>
                    <FormInput id="name" label="Nom" form_type="text" required={true}/>
                    <FormInput id="date" label="Date" form_type="date" required={true}/>
                    <FormInput id="location" label="Lieu" form_type="text" required={true}/>
                    <FormInput id="description" label="Description" form_type="text" required={true}/>
                    <FormInput id="nb_team_per_group" label="Nombre d'équipes par groupe" form_type="text" required={true}/>
                    <FormInput id="phase_qualifications" label="Phase de qualifications" form_type="checkbox" required={false}/>
                    <FormInput id="phase_eliminations" label="Phase d'éliminations" form_type="checkbox" required={false}/>
                </form>
                <hr/>
                <h2>{"Phase de qualifications"}</h2>
                <hr/>
                <h2>{"Phase d'éliminations"}</h2>
                <hr/>
                <Button class="sm:text-xl text-lg px-3 py-2 mx-auto mt-3 mb-16 hover:scale-110 bg-green-700" onclick={on_create_click}>{"Créer un tournoi"}</Button>
            </div>
        </HomeLayout>
    }
}