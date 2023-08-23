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
            <header class="h-16 bg-nut flex items-center">
                <a href="/" class="flex flex-row ml-5 transition-all hover:tracking-widest">
                    <h1 class="mr-5 text-5xl">{"Tournoix"}</h1>
                    <img src="/img/nut.svg" class="h-10"/>
                </a>
            </header>

            <main class={"w-full px-5"}>
                {children.clone()}
            </main>

            <footer></footer>
        </div>
    }
}