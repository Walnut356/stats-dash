#![allow(non_snake_case)]

mod table;
use slpprocess::game::GameStub;
use slpprocess::{parse_stubs, Game};
use table::*;
mod top_bar;
use top_bar::*;
mod cards;
use cards::*;

use std::str::FromStr;
use std::{iter::zip, path::PathBuf};

use dioxus_router::prelude::*;

use dioxus::prelude::*;
use log::LevelFilter;

use polars::prelude::*;
use slpprocess::{parse, stats::StatType, Port};

pub struct WorkingDir(String);
pub struct Stubs(Vec<GameStub>);

pub fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, || WorkingDir("".to_string()));
    use_shared_state_provider(cx, || Stubs(Vec::<GameStub>::new()));
    render! { Router::<Route> {} }
}

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/stats")]
    StatsPage {path: Arc<PathBuf>},
}

#[inline_props]
pub fn Home(cx: Scope) -> Element {
    let working_dir = use_shared_state::<WorkingDir>(cx).unwrap();
    let stubs = use_shared_state::<Stubs>(cx).unwrap();

    cx.render(rsx! {
        head { link { rel: "stylesheet", href: "./dist/pico.classless.min.css" } }
        body {margin: 0, padding: "10px",
            nav { gap: "20px",
                input {
                    flex: "0 0 auto",
                    r#type: "file",
                    id: "file_input",
                    directory: true,
                    overflow: "hidden",
                    max_width: "6.5rem",
                    onchange: |evt| {
                        let fs = evt.files.as_ref().unwrap().files();
                        let file = &fs.get(0);
                        if let Some(f) = file {
                            working_dir.write().0 = f.to_string();
                            stubs.write().0 = parse_stubs(f, true);
                        }
                    }
                }
            }

            for stub in &*stubs.read().0 {
                rsx!(
                    Card { stub: stub.clone()}
                )
            }
        }
    })
}

#[inline_props]
pub fn StatsPage(cx: Scope, path: Arc<PathBuf>) -> Element {
    let game = use_ref(cx, || parse(path.to_str().unwrap(), false).pop().unwrap());
    let port: &UseRef<Port> = use_ref(cx, || Port::P1);
    let stat_type: &UseRef<StatType> = use_ref(cx, || StatType::Defense);

    cx.render(rsx! {
        head { link { rel: "stylesheet", href: "./dist/pico.classless.min.css" } }

        body { margin: 0, padding: "10px", max_height: "100vh",
            div { display: "flex", flex_direction: "column", max_height: "calc(100dvh - 17px)",
                TopBar { players: game.read().players.clone(), port: port, stat_type: stat_type }

                rsx!(
                    p {
                        padding_bottom: 0,
                        margin_bottom: 0,
                        "Game Info:"
                    }
                    figure {
                        overflow: "auto",
                        flex: "1 0 auto",
                    DFTable{ dataframe: game.read().summarize() }
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
                        DFTable{ dataframe: game.read()
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
                                DFTable{ dataframe: game.read()
                                    .player_by_port(*port.read())
                                    .unwrap()
                                    .stats
                                    .get(*stat_type.read())
                                    .unwrap()
                                }
                            }
                        )
                    }
                )
            }
        }
    })
}
