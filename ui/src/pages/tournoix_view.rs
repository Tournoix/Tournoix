use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yew_router::prelude::use_navigator;

use crate::{
    api::{self, models::Tournament},
    components::{
        backlink::Backlink,
        bet_list::BetList,
        bracket::Match,
        groups::{Group, Groups},
        join_code::JoinCode,
        loading_circle::LoadingCircle,
        results::Results,
    },
    layouts::homelayout::HomeLayout,
    routers::Route,
};

#[derive(PartialEq, Properties)]
pub struct TournoixViewProps {
    pub id: i32,
}

#[function_component]
pub fn TournoixView(props: &TournoixViewProps) -> Html {
    let TournoixViewProps { id } = props;
    let navigator = use_navigator().unwrap();

    let tournament: UseStateHandle<Option<Tournament>> = use_state(|| None);
    let loading = use_state(|| true);

    {
        let tournament = tournament.clone();
        let loading = loading.clone();
        let id = id.clone();

        use_effect_once(move || {
            spawn_local(async move {
                tournament.set(api::tournoix::get(id).await.ok());
                loading.set(false);
            });

            || ()
        });
    }

    // TODO Wheter or not the current user can edit this tournament
    let can_edit_tournament = true;

    let groups: UseStateHandle<Vec<Group>> =
        use_state(|| vec![Group {}, Group {}, Group {}, Group {}, Group {}, Group {}]);

    let on_click_edit = {
        let navigator = navigator.clone();
        let id = id.clone();
        Callback::from(move |_| navigator.push(&Route::TournoixEdit { id }))
    };

    let matches: UseStateHandle<Vec<Match>> = use_state(|| {
        vec![
            Match {
                id: 0,
                team1: "Cloud9".to_string(),
                score1: 0,
                team2: "FaZe Clan".to_string(),
                score2: 0,
                started: false,
                finished: false,
            },
            Match {
                id: 1,
                team1: "NaVi".to_string(),
                score1: 0,
                team2: "NRG Esports".to_string(),
                score2: 0,
                started: true,
                finished: false,
            },
            Match {
                id: 2,
                team1: "G2 Esports".to_string(),
                score1: 0,
                team2: "fnatic".to_string(),
                score2: 0,
                started: true,
                finished: true,
            },
            Match {
                id: 3,
                team1: "Team with a comically long name".to_string(),
                score1: 0,
                team2: "Team 42".to_string(),
                score2: 0,
                started: false,
                finished: false,
            },
        ]
    });

    // TODO: DB
    let tournament_over = true;
    let user_nut = 20;

    html! {
        <HomeLayout>
            <div class="flex flex-col items-center h-full pb-16 pt-12 sm:w-9/12 w-11/12 mx-auto relative">
                <Backlink route={Route::Tournoix} label="Retour à la liste des tournoix"/>
                if *loading {
                    <LoadingCircle />
                } else {
                    if tournament.is_none() {
                        <div>{"Oups, ce tournoi n'existe pas :("}</div>
                    } else  {
                        <h1 class="mb-5">{tournament.as_ref().unwrap().name.to_string()}</h1>
                        {if can_edit_tournament { html! {<a onclick={on_click_edit} class="a_link mb-6">{"Modifier ce tournoi"}</a>}} else { html! {} }}
                        <JoinCode code={tournament.as_ref().unwrap().code.to_string()}/>
                        <hr/>
                        <h2>{"Informations"}</h2>
                        <div>{"Date: "}{tournament.as_ref().unwrap().date.unwrap().format("%d.%m.%Y %H:%M:%S")}</div>
                        <div>{"Lieu: "}{tournament.as_ref().unwrap().location.as_ref().unwrap_or(&String::new())}</div>
                        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/ol@v7.2.2/ol.css"/>
                        <script src="https://cdn.jsdelivr.net/npm/ol@v7.2.2/dist/ol.js"></script>
                        <div id="map" class="h-56 w-80"></div>
                        <script>
                        {format!("LOCATION = '{}'", tournament.as_ref().unwrap().location.as_ref().unwrap_or(&String::new()))}
                        </script>
                        <script defer={true}>
    {r#"
                    setTimeout(async () => {
                        const API_KEY_OPEN_CAGE_DATA = '6013e487ba024001bd708e4afd9e3325';
                        const API_KEY_MAPTILER = 'vAS374E4z5f82WH15YOh';

                        const response = await fetch(`https://api.opencagedata.com/geocode/v1/json?q=${LOCATION}&key=${API_KEY_OPEN_CAGE_DATA}&language=fr&pretty=1`);
                        const jsonResponse = await response.json();
                        // HEIG coordinates by default
                        let lat = 46.7788416;
                        let lon = 6.6583221;
                        if (jsonResponse['results'].length > 0) {
                            lat = jsonResponse['results'][0]['geometry'].lat;
                            lon = jsonResponse['results'][0]['geometry'].lng;
                        }
                        const attribution = new ol.control.Attribution({
                            collapsible: false,
                        });

                        const source = new ol.source.TileJSON({
                            url: `https://api.maptiler.com/maps/streets-v2/tiles.json?key=${API_KEY_MAPTILER}`, // source URL
                            tileSize: 512,
                            crossOrigin: 'anonymous'
                        });

                        const map = new ol.Map({
                            layers: [
                                new ol.layer.Tile({
                                    source: source
                                })
                            ],
                            controls: ol.control.defaults.defaults({attribution: false}).extend([attribution]),
                            target: 'map',
                            view: new ol.View({
                                constrainResolution: true,
                                center: ol.proj.fromLonLat([lon, lat]), // starting position [lng, lat]
                                zoom: 14 // starting zoom
                            })
                        });

                        // Add marker
                        var markers = new ol.layer.Vector({
                            source: new ol.source.Vector(),
                            style: new ol.style.Style({
                                image: new ol.style.Icon({
                                    anchor: [0.5, 1],
                                    src: '/img/marker.png'
                                })
                            })
                        });
                        map.addLayer(markers);

                        var marker = new ol.Feature(new ol.geom.Point(ol.proj.fromLonLat([lon, lat])));
                        markers.getSource().addFeature(marker);
                    }, 2000) // Wait so the map library is loaded
"#}</script>
                        <div>{"Description: "}{tournament.as_ref().unwrap().description.to_string()}</div>
                        <hr/>
                        <h2>{"Paris disponibles"}</h2>
                        <p class="discrete">{"Vous pouvez misez vos noix dans ces matchs et peut-être remporter le pactole !"}</p>
                        <p class="mb-4">{format!("Vous possédez actuellement {} noix.", user_nut)}</p>
                        <BetList matches={(*matches).clone()}/>
                        <hr/>
                        <h2>{"Phase de qualifications"}</h2>
                        <ContextProvider<UseStateHandle<Vec<Group>>> context={groups.clone()}>
                            <Groups tournament={tournament.as_ref().unwrap().clone()}/>
                        </ContextProvider<UseStateHandle<Vec<Group>>>>
                        <hr/>
                        <h2>{"Phase d'éliminations"}</h2>
                        /*<Bracket/>*/
                        if tournament_over {
                            <hr/>
                            <h2>{"Résultats"}</h2>
                            <Results tournament_id={ id }/>
                        }
                    }
                }
            </div>
        </HomeLayout>
    }
}
