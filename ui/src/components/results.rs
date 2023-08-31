use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ResultsProps {}

#[function_component]
pub fn Results(props: &ResultsProps) -> Html {
    let ResultsProps {} = props;

    html! {
        <div class="bg-green-200">
            {"Results component"}
            <div class="text-red-500">{"AFFICHER UNIQUEMENT SI TOUT LES MATCHS DE CE TOURNOIS SONT FERMÉS"}</div>
            <div class="flex">
                <div class="m-4 p-4 bg-nutLight">
                    <h3>{"Classement des équipes"}</h3>
                    <ul>
                        <li>{"1: Equipe A"}</li>
                        <li>{"2: Equipe B"}</li>
                        <li>{"3: Equipe C"}</li>
                        <li>{"4: Equipe D"}</li>
                    </ul>
                </div>
                <div class="m-4 p-4 bg-nutLight">
                    <h3>{"Classement des parieurs"}</h3>
                    <ul>
                        <li>{"1: Joueur A - 321 noix"}</li>
                        <li>{"2: Joueur B - 256 noix"}</li>
                        <li>{"3: Joueur C - 119 noix"}</li>
                        <li>{"4: Joueur D - 65 noix"}</li>
                    </ul>
                </div>
                <div class="flex flex-col gap-12">
                    <div class="flex flex-col justify-center items-center m-4">
                        <h2>{"Equipe A"}</h2>
                        <h2>{"est la noisette vainqueur !"}</h2>
                        <img class="w-16" src="/img/cup_first.png"/>
                    </div>
                    <div class="flex flex-col justify-center items-center m-4">
                        <h2>{"Joueur A"}</h2>
                        <h2>{"est la plus grosse noix !"}</h2>
                        <div class="flex">
                            <span class="text-3xl">{"321"}</span>
                            <img class="w-16" src="/img/nut.svg"/>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}