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
        user_provider::UserContext, button::Button,
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
    
    let user_info = use_context::<UserContext>().expect("Missing user context provider");
    let tournament: UseStateHandle<Option<Tournament>> = use_state(|| None);
    let has_joined_this_tournament = use_state(|| false);
    let loading = use_state(|| true);
    let user_nut = use_state(|| 0);
    let can_edit_tournament = use_state(|| false);
    let tournament_is_started = use_state(|| false);

    {
        let tournament = tournament.clone();
        let user_info = user_info.clone();
        let user = user_info.user.clone();
        let user_nut = user_nut.clone();
        let loading = loading.clone();
        let id = id.clone();
        let has_joined_this_tournament = has_joined_this_tournament.clone();
        let can_edit_tournament = can_edit_tournament.clone();

        use_effect_once(move || {
            if let Some(user) = user {
                {
                    let user = user.clone();
                    let user_nut = user_nut.clone();
                    let tournament_clone = tournament.clone();

                    spawn_local(async move {
                        tournament_clone.set(api::tournoix::get(id).await.ok());
                        loading.set(false);
                    });

                    let tournament = tournament.clone();
                    let can_edit_tournament = can_edit_tournament.clone();
        
                    spawn_local(async move {
                        if let Some(subscriptions) = user.subscriptions().await.ok() {
                            subscriptions.iter().for_each(|t| {
                                if t.id == id {
                                    has_joined_this_tournament.set(true);
                                }
                            });
                        }
                    });
                }
            }

            || ()
        });
    }

    // Bettable games
    let trigger = use_state(|| false);
    let bettable_games = use_state(|| Vec::new());
    let loading_bettable_games = use_state(|| true);
    {
        let bettable_games = bettable_games.clone();
        let loading_bettable_games = loading_bettable_games.clone();
        let tournament_clone = (*tournament).clone();
        let user_nut = user_nut.clone();
        let can_edit_tournament = can_edit_tournament.clone();
        let tournament_is_started = tournament_is_started.clone();

        use_effect_with_deps(
            move |_| {
                if let Some(tournament_clone) = tournament_clone {
                    let tournament_clone = tournament_clone.clone();
                    let tournament = tournament_clone.clone();
                    let user_nut = user_nut.clone();
                    let can_edit_tournament = can_edit_tournament.clone();
                    let tournament_is_started = tournament_is_started.clone();
                    spawn_local(async move {
                        if let Some(games) = tournament_clone.get_matches().await.ok() {
                            bettable_games.set(games.iter()
                                .filter(|&m| m.status == 0 || 1 == 1) // filter out finished matches
                                .cloned()
                                .collect()
                            );
                        }
                        loading_bettable_games.set(false);

                        if let Some(nut) = api::game::get_nb_nut(tournament_clone.id).await.ok() {
                            user_nut.set(nut.stock);
                        }
                    });

                    spawn_local(async move {
                        if let Some(can_edit) = api::tournoix::is_tournoix_owner(tournament.id.clone()).await.ok() {
                            can_edit_tournament.set(can_edit);
                        }
                    });

                    spawn_local(async move {
                        if let Some(is_started) = api::tournoix::is_tournoix_started(tournament.id.clone()).await.ok() {
                            tournament_is_started.set(is_started);
                        }
                    });
                }

                || ()
            },
            (tournament.clone(), trigger.clone()),
        );
    }
    let on_click_refresh_games = {
        let trigger = trigger.clone();
        Callback::from(move |_| trigger.set(!*trigger))
    };

    let groups: UseStateHandle<Vec<Group>> =
        use_state(|| vec![Group {}, Group {}, Group {}, Group {}, Group {}, Group {}]);

    let on_click_edit = {
        let navigator = navigator.clone();
        let id = id.clone();
        Callback::from(move |_| navigator.push(&Route::TournoixEdit { id }))
    };

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
                        {if !(*can_edit_tournament) { html! {<a onclick={on_click_edit} class="a_link mb-6">{"Modifier ce tournoi"}</a>}} else { html! {} }}
                        <JoinCode code={tournament.as_ref().unwrap().code.to_string()}/>
                        <hr/>
                        <h2>{"Informations"}</h2>
                        if tournament.as_ref().unwrap().is_closed {
                            <div class="text-lg">{"Etat: Ce tournoi est fermé."}</div>
                        } else {
                            if *tournament_is_started {
                                <div class="text-lg">{"Etat: Ce tournoi a démarré."}</div>
                            } else {
                                <div class="text-lg">{"Etat: Ce tournoi n'est pas encore démarré."}</div>
                            }
                        }
                        <div>{"Date: "}{tournament.as_ref().unwrap().date.format("%d.%m.%Y %H:%M:%S")}</div>
                        <div>{"Lieu: "}{tournament.as_ref().unwrap().location.as_ref().unwrap_or(&String::new())}</div>
                        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/ol@v7.2.2/ol.css"/>
                        <script src="https://cdn.jsdelivr.net/npm/ol@v7.2.2/dist/ol.js"></script>
                        <div id="map" class="h-56 w-80" style="background-image: url(\"/img/loading.gif\")"></div>
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
                        if *has_joined_this_tournament {
                            <p class="discrete">{"Vous pouvez misez vos noix dans ces matchs et peut-être remporter le pactole !"}</p>
                            <p class="mb-2">{format!("Vous possédez actuellement {} noix.", &*user_nut)}</p>
                            <Button class="px-3 py-2 hover:scale-110 mb-4" onclick={on_click_refresh_games}>{"Rafraîchir"}</Button>
                            if *loading_bettable_games {
                                <LoadingCircle />
                            } else {
                                <BetList tournament_id={id.clone()} matches={(*bettable_games).clone()}/>
                            }
                        } else {
                            {"Vous devez rejoindre ce tournoi afin de pouvoir y miser vos noix."}
                        }
                        <hr/>
                        <h2>{"Phase de qualifications"}</h2>
                        <ContextProvider<UseStateHandle<Vec<Group>>> context={groups.clone()}>
                            <Groups tournament={tournament.as_ref().unwrap().clone()}/>
                        </ContextProvider<UseStateHandle<Vec<Group>>>>
                        <hr/>
                        <h2>{"Phase d'éliminations"}</h2>
                        /*<Bracket/>*/
                        <hr/>
                        <h2>{"Résultats"}</h2>
                        <Results can_show_results={tournament.as_ref().unwrap().is_closed && *tournament_is_started} tournament_id={ id }/>
                    }
                }
            </div>
        </HomeLayout>
    }
}
