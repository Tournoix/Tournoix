use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct BracketMatchProps {}

#[function_component]
pub fn BracketMatch(props: &BracketMatchProps) -> Html {
    let BracketMatchProps {} = props;

    html! {
        <>
            <li class={"game game-top"}>
                <div class={"bg-nutLighter pl-2"}>
                    {"Team 1"}
                    <span class="bg-red-300">{42}</span>
                </div>
            </li>
            <li class={"game game-spacer"}>{"\u{00a0}"}</li>
            <li class={"game game-bottom"}>
                <div class={"bg-nutLighter font-bold pl-2"}>
                    {"Team 2"}
                    <span class="bg-green-300">{69}</span>
                </div>
            </li>
        </>
    }
}