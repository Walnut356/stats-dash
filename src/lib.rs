#![allow(non_snake_case)]

mod table;
use table::*;

use std::{iter::zip, path::PathBuf};
use std::str::FromStr;

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

fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        head { link { rel: "stylesheet", href: "./dist/pico-1.5.9/css/pico.classless.min.css" } }
    })
}

#[inline_props]
pub fn Table(cx: Scope, path: String) -> Element {
    render! {"eef"}
}

#[inline_props]
pub fn Home(cx: Scope) -> Element {
    let connect_code = use_ref(cx, String::default);
    let game: &UseRef<Option<slpprocess::Game>> = use_ref(cx, || None);
    let df = use_ref(cx, DataFrame::default);
    let stat_type = use_ref(cx, || StatType::Defense);
    let path = use_ref(cx, String::default);
    let match_id = use_ref(cx, String::default);
    let players = use_ref(cx, Vec::<String>::new);
    let port_selection = use_state(cx, || Port::P1);

    let regex = r"\w{1,4}#\d{1,3}";

    cx.render(rsx! {
        Header {}
        body {
            input {
                // tell the input to pick a file
                r#type: "file",
                // list the accepted extensions
                accept: ".slp",
                // pick multiple files
                multiple: false,
                onchange: |ev| {
                    if let Some(file_engine) = &ev.files {
                        let files = file_engine.files();
                        if !files.is_empty() {
                            path.set(files[0].clone());
                            let temp = parse(&files[0]).pop().unwrap();
                            match_id.set(temp.metadata.match_id.clone().unwrap_or_default());
                            players
                                .set(
                                    vec![
                                        temp.players[0].connect_code.clone().unwrap_or_default(),
                                        temp.players[1].connect_code.clone().unwrap_or_default(),
                                    ],
                                );
                            df.set(
                                temp
                                    .player_by_port(**port_selection)
                                    .unwrap()
                                    .stats
                                    .get(*stat_type.read())
                                    .unwrap(),
                            );
                            game.set(Some(temp));
                        }
                    }
                }
            }

            if game.read().is_some() {
                cx.render(rsx!(
                    DFTable { dataframe: game.read().as_ref().unwrap().summarize() }
                ))

            }

            fieldset {
                legend { "Select a player to view stats:" }
                div {
                    input {
                        value: "1",
                        r#type: "radio",
                        name: "port_radio",
                        checked: "true",
                        id: "player_1",
                        onclick: |ev| {
                            if game.read().is_some() {
                                let port = game.read().as_ref().unwrap().players[0].port;
                                if **port_selection != port {
                                    port_selection.set(port);
                                    df.set(
                                        game
                                            .read()
                                            .as_ref()
                                            .unwrap()
                                            .player_by_port(port)
                                            .unwrap()
                                            .stats
                                            .get(*stat_type.read())
                                            .unwrap(),
                                    );
                                }
                            }
                        }
                    }
                    // ew rust please
                    label { r#for: "player_1",
                        r#"{players.read().get(0).map(|x| x.as_str()).unwrap_or_else(|| "Player 1")}"#
                    }
                }

                div {
                    input {
                        r#type: "radio",
                        value: "2",
                        name: "port_radio",
                        id: "player_2",
                        onclick: |ev| {
                            if game.read().is_some() {
                                let port = game.read().as_ref().unwrap().players[1].port;
                                if **port_selection != port {
                                    port_selection.set(port);
                                    df.set(
                                        game
                                            .read()
                                            .as_ref()
                                            .unwrap()
                                            .player_by_port(port)
                                            .unwrap()
                                            .stats
                                            .get(*stat_type.read())
                                            .unwrap(),
                                    );
                                }
                            }
                            println!("DF changed")
                        }
                    }

                    label { r#for: "player_2",
                        r#"{players.read().get(1).map(|x| x.as_str()).unwrap_or_else(|| "Player 2")}"#
                    }
                }
            }

            select {
                onchange: move |ev| {
                    stat_type.set(StatType::from_str(&ev.value).unwrap());
                    if game.read().is_some() {
                        df.set(
                            game
                                .read()
                                .as_ref()
                                .unwrap()
                                .player_by_port(**port_selection)
                                .unwrap()
                                .stats
                                .get(StatType::from_str(&ev.value).unwrap())
                                .unwrap(),
                        );
                    }
                },
                option { "Defense" }
                option { "Wavedash" }
                option { "Input" }
                option { "L Cancel" }
                option { "Item" }
            }
        }

        DFTable { dataframe: df.read().clone() }
    })
}
