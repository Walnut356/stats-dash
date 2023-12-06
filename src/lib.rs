#![allow(non_snake_case)]

mod table;
use slpprocess::player::Player;
use slpprocess::Game;
use table::*;
mod top_bar;
use top_bar::*;

use std::str::FromStr;
use std::{iter::zip, path::PathBuf};

use dioxus_router::prelude::*;

use dioxus::prelude::*;
use log::LevelFilter;

use polars::prelude::*;
use slpprocess::{parse, stats::StatType, Port};

pub fn app(cx: Scope) -> Element {
    render! { Router::<Route> {} }
}

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/:path")]
    Table { path: String },
}

#[inline_props]
pub fn Table(cx: Scope, path: String) -> Element {
    render! {"eef"}
}

#[inline_props]
pub fn Home(cx: Scope) -> Element {
    let games: &UseRef<Option<Vec<Game>>> = use_ref(cx, || None);
    let port: &UseRef<Port> = use_ref(cx, || Port::P1);
    let stat_type: &UseRef<StatType> = use_ref(cx, || StatType::Defense);

    cx.render(rsx! {
        head { link { rel: "stylesheet", href: "./dist/pico.classless.min.css" } }

        body { margin: 0, padding: "10px", max_height: "100vh",
            div { display: "flex", flex_direction: "column", max_height: "calc(100dvh - 17px)",
                TopBar { games: games, port: port, stat_type: stat_type }
                if games.read().is_some() {
                    rsx!(
                        p {
                            padding_bottom: 0,
                            margin_bottom: 0,
                            "Game Info:"
                        }
                        figure {
                            overflow: "auto",
                            flex: "1 0 auto",
                        DFTable{ dataframe: games.read().as_ref().unwrap().get(0).unwrap().summarize() }
                        }

                        p {
                            padding_bottom: 0,
                            margin_bottom: 0,
                            "Summary:"
                        }
                        figure {
                            bottom: 0,
                            overflow: "auto",
                            flex: "1 0 auto",
                            margin: 0,
                            padding: 0,
                            // max_height: "50vh",
                            DFTable{ dataframe: games
                                .read()
                                .as_ref()
                                .unwrap()
                                .get(0)
                                .unwrap()
                                .player_by_port(*port.read())
                                .unwrap()
                                .stats
                                .get_summary(*stat_type.read())
                                .unwrap_or_default()
                            }
                        }

                        if ![StatType::Input, StatType::Item].contains(&stat_type.read()) {
                            rsx!(
                                p {
                                padding_bottom: 0,
                                margin_bottom: 0,
                                "Events:"
                                }
                                figure {
                                    bottom: 0,
                                    overflow: "auto",
                                    flex: "1 1 auto",
                                    margin: 0,
                                    padding: 0,
                                    // max_height: "50vh",
                                    DFTable{ dataframe: games
                                        .read()
                                        .as_ref()
                                        .unwrap()
                                        .get(0)
                                        .unwrap()
                                        .player_by_port(*port.read())
                                        .unwrap()
                                        .stats
                                        .get(*stat_type.read())
                                        .unwrap()
                                    }
                                }
                            )
                        }
                    )}
            }
        }
    })
}
