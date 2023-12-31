use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{components::button::Button, layouts::homelayout::HomeLayout, routers::Route};

#[derive(PartialEq, Properties)]
pub struct HomeProps {}

#[function_component]
pub fn Home(props: &HomeProps) -> Html {
    let HomeProps {} = props;
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Tournoix));

    html! {
        <HomeLayout>
            <div class="h-full relative">
                <div class="wavy absolute top-0 w-full h-full pointer-events-none opacity-30"></div>
                <div class="font-bebas flex flex-col items-center h-full sm:w-9/12 w-11/12 mx-auto relative">
                    /*<img src="/img/bullets_texture.svg" class="absolute opacity-5 w-full"/>*/
                    <img src="/img/hero_nut.png" class="sm:h-32 h-24 mt-16 mb-8 drop-shadow"/>

                    <div class="flex flex-col items-center space-y-2">
                        <h1 class="whitespace-nowrap">{"Arrêtez de vous les briser,"}</h1>
                        <h1 class="whitespace-nowrap">{"utilisez Tournoix"}</h1>
                    </div>

                    <div class="mt-24 mb-6">
                        <h2 class="text-center">{"Principe"}</h2>
                        <div class="ml-5">
                            <p class="text-center">{"Lorsque vous organisez votre tournoi sur Tournoix, vos utilisateurs peuvent voir l'avancement du tournoi en live et miser une monnaie virtuelle (des noix) sur l'équipe gagnante."}</p>
                            <p class="text-center">{"A la fin du tournoi, un classement affiche l'équipe gagnante ainsi que l'utilisateur ayant le plus de noix. Tout le monde commence avec 20 noix par tournoi."}</p>
                        </div>
                    </div>

                    <Button class="flex items-center space-x-2 mt-4 text-2xl mb-32 px-5 py-3 hover:scale-125" {onclick}>
                        <span>{"Créer un tournoi maintenant"}</span>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M11.25 4.5l7.5 7.5-7.5 7.5m-6-15l7.5 7.5-7.5 7.5" />
                        </svg>
                    </Button>

                    <div class="mt-5 mb-16">
                        <h2 class="text-center">{"Pourquoi utiliser Tournoix ?"}</h2>
                        <ul class="flex flex-wrap justify-center">
                            <li class="why-card">
                                <h3>{"Engagement"}</h3>
                                <p>{"Faites participer vos utilisateurs et incitez-les à l'action."}</p>
                            </li>
                            <li class="why-card">
                                <h3>{"Communauté"}</h3>
                                <p>{"Rassemblez vos utilisateurs dans une communauté où ils pourront comparer leur score."}</p>
                            </li>
                            <li class="why-card">
                                <h3>{"Gamification"}</h3>
                                <p>{"En gamifiant votre tournoi, vous donnez un coup de jeune à votre image."}</p>
                            </li>
                        </ul>
                    </div>

                    <div class="mt-5 mb-32">
                        <h2 class="text-center">{"Notre équipe"}</h2>
                        <ul class="flex flex-wrap justify-center">
                            <li class="team-card">
                                <div class="flex flex-row">
                                    <img src="/img/nut_almond.jpeg" class="team-nut-image"/>
                                    <div>
                                        <h3 class="mb-0">{"Dorian"}</h3>
                                        <p class="discrete">{"Grand manie-tout"}</p>
                                    </div>
                                </div>
                                <p>{"Responsable architecture"}</p>
                            </li>
                            <li class="team-card">
                                <div class="flex flex-row">
                                    <img src="/img/nut_coco.jpeg" class="team-nut-image"/>
                                    <div>
                                        <h3 class="mb-0">{"Joris"}</h3>
                                        <p class="discrete">{"L'archiviste"}</p>
                                    </div>
                                </div>
                                <p>{"Responsable base de donnée"}</p>
                            </li>
                            <li class="team-card">
                                <div class="flex flex-row">
                                    <img src="/img/nut_acorn.jpeg" class="team-nut-image"/>
                                    <div>
                                        <h3 class="mb-0">{"Leandro"}</h3>
                                        <p class="discrete">{"Artiste"}</p>
                                    </div>
                                </div>
                                <p>{"Responsable frontend"}</p>
                            </li>
                            <li class="team-card">
                                <div class="flex flex-row">
                                    <img src="/img/nut_peanut.jpeg" class="team-nut-image"/>
                                    <div>
                                        <h3 class="mb-0">{"Rhyan"}</h3>
                                        <p class="discrete">{"R-h-y-a-n, pas R-y-a-n"}</p>
                                    </div>
                                </div>
                                <p>{"Responsable backend"}</p>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        </HomeLayout>
    }
}
