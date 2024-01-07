use std::collections::{HashSet, HashMap};

use counter::Counter;
use dioxus::{prelude::*, html::SvgAttributes};
use dioxus_router::prelude::*;
use slpprocess::game::{Game, GameStub, GameMetadata};
use ssbm_utils::enums::{Character, Costume, StageID};
use time::UtcOffset;

use crate::{app::{Route, Stubs, Filters}, utils::filter_game, components::StockIcon};

#[component]
pub fn Card(cx: Scope, set: (usize, usize), filters: Filters) -> Element {
    let stubs = use_shared_state::<Stubs>(cx).unwrap();
    let games = &stubs.read().0[set.0..set.1];
    let stub = games[0].clone();

    let full_set: &UseRef<Vec<Game>> = use_ref(cx, Vec::new);

    let date = stub.date;

    let date_str = format!(
        "{} {} {}, {}",
        date.weekday(),
        date.month(),
        date.day(),
        date.year(),
    );

    let p1 = &stub.players[0];
    let p2 = &stub.players[1];

    let mut p1_map: HashMap<Character, Counter<Costume>> = HashMap::new();
    let mut p2_map: HashMap<Character, Counter<Costume>> = HashMap::new();



    for game in games {
        *p1_map.entry(game.players[0].character).or_default().entry(game.players[0].costume).or_default() += 1;
        *p2_map.entry(game.players[1].character).or_default().entry(game.players[1].costume).or_default() += 1;
        // if p1_chars.contains_key(&game.players[0].character) {
        //     p1_chars[&game.players[0].character][&game.players[0].costume] += 1;
        // } else {

        // }
    }
    let mut p1_chars = Vec::new();
    for (k, v) in p1_map {
        p1_chars.push((k, v.total::<usize>(), v.most_common()[0].0))
    }

    p1_chars.sort_by(|a, b| b.1.cmp(&a.1));

    let mut p2_chars = Vec::new();
    for (k, v) in p2_map {
        p2_chars.push((k, v.total::<usize>(), v.most_common()[0].0))
    }

    p2_chars.sort_by(|a, b| b.1.cmp(&a.1));

    // let p1_chars = games
    //     .iter()
    //     .map(|g| g.players[0].character)
    //     .collect::<Counter<_>>().most_common();
    // let p2_chars = games
    //     .iter()
    //     .map(|g| g.players[1].character)
    //     .collect::<Counter<_>>().most_common();

    let navigator = use_navigator(cx);

    let filtered = if stubs.read().0[set.0..set.1].iter().any(|g| filter_game(g, filters)) { "initial" } else { "none" };


    render!(
        div {
            display: "{filtered}",
        details {
            summary { class: "none",
                article { class: "background no-elevate",
                    onclick: |_evt| {
                        to_owned!(stubs);
                        if full_set.read().is_empty() {
                            *full_set.write() = stubs.read().0[set.0..set.1].iter().map(Into::<Game>::into).collect();
                        }
                    },
                    div { class: "row",
                        p { class: "max", color: "#999", "{format_time(&stub)}" }
                        p { class: "max", color: "#999", "{date_str}" }
                        p { color: "#999", "{games[0].match_type().to_string()}" }
                    }
                    nav { class: "center-align medium-space",
                        i { visibility: "hidden", "expand_more" }
                        div { class: "max",
                            div {
                                for (chr,_, cost) in p1_chars {
                                    render!(
                                        StockIcon { character: (chr, cost)}
                                    )
                                }
                            }
                            div {
                                b { "{p1.display_name.clone().unwrap_or_else(|| p1.port.to_string())} " }
                                p { color: "#999", font_size: "80%",
                                    "{p1.connect_code.clone().unwrap_or_default()}"
                                }
                            }
                        }
                        div { class: "max",
                            div {
                                for (chr, _, cost) in p2_chars {
                                    render!(
                                        StockIcon { character: (chr, cost)}
                                    )
                                }
                            }
                            div {
                                b { "{p2.display_name.clone().unwrap_or_else(|| p2.port.to_string())} " }
                                p { color: "#999", font_size: "80%",
                                    "{p2.connect_code.clone().unwrap_or_default()}"
                                }
                            }
                        }
                        i { "expand_more" }
                    }
                }
            }
            table { class: "stripes center-align",
                tbody {
                    for (i, game) in full_set.read().iter().enumerate() {
                        render!(
                        // println!("./dist/Stages/{}.png", game.stage().to_string())
                            tr {
                                onclick: move |_| {navigator.push(Route::StatsPage { index: i + set.0 });},
                                td {
                                    text_align: "right",
                                    StockIcon { character: (game.players[0].character, game.players[0].costume), opacity: if game.players[0].frames.post.stocks[game.players[0].frames.len() - 1] == 4 { 1.0 } else { 0.25 }}
                                    StockIcon { character: (game.players[0].character, game.players[0].costume), opacity: if game.players[0].frames.post.stocks[game.players[0].frames.len() - 1] >= 3 { 1.0 } else { 0.25 }}
                                    StockIcon { character: (game.players[0].character, game.players[0].costume), opacity: if game.players[0].frames.post.stocks[game.players[0].frames.len() - 1] >= 2 { 1.0 } else { 0.25 }}
                                    StockIcon { character: (game.players[0].character, game.players[0].costume), opacity: if game.players[0].frames.post.stocks[game.players[0].frames.len() - 1] >= 1 { 1.0 } else { 0.25 }}
                                    // img {
                                    //     height: "24px",
                                    //     width: "24px",
                                    //     src: "./dist/Characters/{game.players[0].character.to_string()}/{game.players[0].costume.to_string()}.png",
                                    //     opacity: "{(game.players[0].frames.post.stocks[game.players[0].frames.len() - 1] == 4).then_some(1.0).unwrap_or(0.5)}",
                                    // }
                                    // img {
                                    //     height: "24px",
                                    //     width: "24px",
                                    //     src: "./dist/Characters/{game.players[0].character.to_string()}/{game.players[0].costume.to_string()}.png",
                                    //     opacity: "{(game.players[0].frames.post.stocks[game.players[0].frames.len() - 1] >= 3).then_some(1.0).unwrap_or(0.5)}",
                                    // }
                                    // img {
                                    //     height: "24px",
                                    //     width: "24px",
                                    //     src: "./dist/Characters/{game.players[0].character.to_string()}/{game.players[0].costume.to_string()}.png",
                                    //     opacity: "{(game.players[0].frames.post.stocks[game.players[0].frames.len() - 1] >= 2).then_some(1.0).unwrap_or(0.5)}",
                                    // }
                                    // img {
                                    //     height: "24px",
                                    //     width: "24px",
                                    //     src: "./dist/Characters/{game.players[0].character.to_string()}/{game.players[0].costume.to_string()}.png",
                                    //     opacity: "{(game.players[0].frames.post.stocks[game.players[0].frames.len() - 1] >= 1).then_some(1.0).unwrap_or(0.5)}",
                                    // }
                                }
                                td {

                                    p { "{format_time(game)}"}
                                    img {
                                        height: "48px",
                                        width: "64px",
                                        src: "./dist/Stages/{game.stage().to_string()}.png"
                                    }
                                    p { "{format_duration(game)}"}
                                }
                                td{
                                    text_align: "left",
                                    StockIcon { character: (game.players[1].character, game.players[1].costume), opacity: if game.players[1].frames.post.stocks[game.players[1].frames.len() - 1] >= 1 { 1.0 } else { 0.25 }}
                                    StockIcon { character: (game.players[1].character, game.players[1].costume), opacity: if game.players[1].frames.post.stocks[game.players[1].frames.len() - 1] >= 2 { 1.0 } else { 0.25 }}
                                    StockIcon { character: (game.players[1].character, game.players[1].costume), opacity: if game.players[1].frames.post.stocks[game.players[1].frames.len() - 1] >= 3 { 1.0 } else { 0.25 }}
                                    StockIcon { character: (game.players[1].character, game.players[1].costume), opacity: if game.players[1].frames.post.stocks[game.players[1].frames.len() - 1] == 4 { 1.0 } else { 0.25 }}
                                    // img {
                                    //     height: "24px",
                                    //     width: "24px",
                                    //     src: "./dist/Characters/{game.players[1].character.to_string()}/{game.players[1].costume.to_string()}.png",

                                    //     opacity: "{(game.players[1].frames.post.stocks[game.players[1].frames.len() - 1] >= 1).then_some(1.0).unwrap_or(0.5)}",
                                    // }
                                    // img {
                                    //     height: "24px",
                                    //     width: "24px",
                                    //     src: "./dist/Characters/{game.players[1].character.to_string()}/{game.players[1].costume.to_string()}.png",

                                    //     opacity: "{(game.players[1].frames.post.stocks[game.players[1].frames.len() - 1] >= 2).then_some(1.0).unwrap_or(0.5)}",
                                    // }
                                    // img {
                                    //     height: "24px",
                                    //     width: "24px",
                                    //     src: "./dist/Characters/{game.players[1].character.to_string()}/{game.players[1].costume.to_string()}.png",

                                    //     opacity: "{(game.players[1].frames.post.stocks[game.players[1].frames.len() - 1] >= 3).then_some(1.0).unwrap_or(0.5)}",
                                    // }
                                    // img {
                                    //     height: "24px",
                                    //     width: "24px",
                                    //     src: "./dist/Characters/{game.players[1].character.to_string()}/{game.players[1].costume.to_string()}.png",

                                    //     opacity: "{(game.players[1].frames.post.stocks[game.players[1].frames.len() - 1] == 4).then_some(1.0).unwrap_or(0.5)}",
                                    // }
                                }
                            }
                        )
                    }
                }
            }

        }
        div { class: "space"}
    }
    )
}

fn stage_string(stage: StageID) -> &'static str {
    match stage {
        StageID::FOUNTAIN_OF_DREAMS => "Fountain of Dreams",
        StageID::POKEMON_STADIUM => "Pokemon Stadium",
        StageID::YOSHIS_STORY => "Yoshi's Story",
        StageID::DREAM_LAND_N64 => "Dreamland",
        StageID::BATTLEFIELD => "Battlefield",
        StageID::FINAL_DESTINATION => "Final Destination",
        _ => stage.into(),
    }
}

fn format_time<G: GameMetadata>(game: &G) -> String {
    let time = game
        .date()
        .to_offset(UtcOffset::current_local_offset().unwrap())
        .time();

    // this whole thing is pretty ick but oh well
    let hour = time.hour();
    let time_str = format!(
        "{: >2}:{:0>2}:{:0>2}{}",
        match hour {
            x if x > 12 => x - 12,
            0 => 12,
            x => x,
        },
        time.minute(),
        time.second(),
        match hour {
            _ if hour >= 12 => "pm",
            _ => "am",
        },
    );

    time_str
}

fn format_duration<G: GameMetadata>(game: &G) -> String {
    let mins = game.duration().as_secs() / 60;
    format!("{}m{:0>2}s", mins, game.duration().as_secs() - (60 * mins))
}
