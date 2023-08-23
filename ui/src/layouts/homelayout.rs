use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct HomeLayoutProps {
    pub children: Children,
}

#[function_component]
pub fn HomeLayout(props: &HomeLayoutProps) -> Html {
    let HomeLayoutProps { children } = props;

    html! {
        <div>
            <header></header>

            <main class={"w-full px-5"}>
                {children.clone()}
            </main>

            <footer></footer>
        </div>
    }
}