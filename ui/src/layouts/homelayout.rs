use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct HomeLayoutProps {
    pub children: Children,
}

#[function_component]
pub fn HomeLayout(props: &HomeLayoutProps) -> Html {
    let HomeLayoutProps { children } = props;

    html! {
        <div class="font-bebas">
            <header class="h-16 bg-light flex items-center drop-shadow-lg pl-8">
                <a href="/" class="flex flex-row ml-5 transition-all hover:tracking-[.2em] hover:duration-[200ms] duration-[400ms]">
                    <img src="/img/nut.svg" class="h-12 mr-8"/>
                    <h1 class="text-5xl">{"Tournoix"}</h1>
                </a>
            </header>

            <main class={"w-full"}>
                {children.clone()}
            </main>

            <footer class="bg-nutDark absolute w-full text-white flex text-lg align-center justify-center pt-3">
                <div class="relative left-[-40%]">
                    <h3 class="text-2xl mb-1">{"A propos"}</h3>
                    <ul>
                        <li>{"L'équipe"}</li>
                        <li>{"Contact"}</li>
                        <li>{"Localisation"}</li>
                    </ul>
                </div>
                <div class="relative right-[-40%] text-right">
                    <img src="/img/nut.svg" class="h-8 invert ml-auto"/>
                    <p>{"Copyright Tournoix"}</p>
                    <p>{"© 2023"}</p>
                </div>
            </footer>
        </div>
    }
}