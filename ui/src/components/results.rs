use yew::prelude::*;

use crate::utils::utils::team_color_wrapper;

#[derive(PartialEq, Clone)]
pub struct Score {
    name: String,
    val: i32,
}

#[derive(PartialEq, Properties)]
pub struct ResultsProps {
    pub tournament_id: i32,
}

#[function_component]
pub fn Results(props: &ResultsProps) -> Html {
    let ResultsProps { tournament_id } = props;

    let teams_score = use_state(|| vec![
        Score { name: "fnatic".to_string(), val: 1 },
        Score { name: "FaZe Clan".to_string(), val: 4 },
        Score { name: "G2 Esports".to_string(), val: 3 },
        Score { name: "Cloud9".to_string(), val: 2 },
    ]);
    let gamblers_score = use_state(|| vec![
        Score { name: "Jean-Paul".to_string(), val: 265 },
        Score { name: "Alex Terrieur".to_string(), val: 424 },
        Score { name: "pampe-lemousse".to_string(), val: 15 },
        Score { name: "N1_2_jardin".to_string(), val: 986 },
    ]);

    {
        let teams_score = teams_score.clone();
        let gamblers_score = gamblers_score.clone();
        use_effect_with_deps(
            move |_| {
                // Proposition for DB connection:
                //async move {
                //    let tournament = fetch_tournament(*tournament_id).await;
                //    teams_score.set(tournament.teams_score);
                //    gamblers_score.set(tournament.gamblers_score);
                //})

                // Sort by score value
                let mut teams_score_copy: Vec<Score> = vec![];
                let mut gamblers_score_copy: Vec<Score> = vec![];
                for score in (*teams_score).iter() {
                    teams_score_copy.push((*score).clone());
                }
                for score in (*gamblers_score).iter() {
                    gamblers_score_copy.push((*score).clone());
                }

                // Sort by rank, smaller is better
                teams_score_copy.sort_by(|a, b| a.val.cmp(&b.val));

                // Sort by nut number, bigger is better
                gamblers_score_copy.sort_by(|a, b| b.val.cmp(&a.val));

                teams_score.set(teams_score_copy);
                gamblers_score.set(gamblers_score_copy);
            },
            (*tournament_id).clone(),
        );
    }

    html! {
        <div class="flex">
            <div class="p-4 bg-nutLight">
                <h3>{"Classement des équipes"}</h3>
                <ul class="h-96 overflow-y-scroll">
                    {
                        teams_score.iter().enumerate().map(|(index, score)| {
                            html!{<li style={team_color_wrapper((*score).name.clone())} class="team-border-color border-r-8 px-2 m-2 bg-nutLighter">
                                {format!("{}) {}", (index + 1).to_string(), score.name)}
                            </li>}
                        }).collect::<Html>()
                    }
                </ul>
            </div>
            <div class="ml-4 p-4 bg-nutLight">
                <h3>{"Classement des parieurs"}</h3>
                <ul class="h-96 overflow-y-scroll">
                    {
                        gamblers_score.iter().enumerate().map(|(index, score)| {
                            html!{<li class="px-2 m-2 bg-nutLighter">
                                {format!("{}) {} - {}", (index + 1).to_string(), score.name, score.val.to_string())}
                            </li>}
                        }).collect::<Html>()
                    }
                </ul>
            </div>
            <div class="flex flex-col gap-12">
                <div class="flex flex-col justify-center items-center m-4">
                if let Some(winning_team_score) = teams_score.get(0) {
                    <h2 style={team_color_wrapper(winning_team_score.name.clone())} class="team-text-color">{winning_team_score.name.clone()}</h2>
                    <h2>{"est l'équipe vainqueur !"}</h2>
                    <img class="w-24 wiggle" src="/img/cup_first.png"/>
                }
                </div>
                <div class="flex flex-col justify-center items-center m-4">
                    if let Some(winning_gambler_score) = gamblers_score.get(0) {
                        <h2>{winning_gambler_score.name.clone()}</h2>
                        <h2>{"a le plus de noix !"}</h2>
                        <div class="flex items-center">
                            <span class="text-3xl mr-2">{winning_gambler_score.val.clone().to_string()}</span>
                            <img class="w-14" src="/img/nut.svg"/>
                        </div>
                    }
                </div>
            </div>
        </div>
    }
}
