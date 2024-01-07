use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use slpprocess::{stats::StatType, Game, Port};

use crate::{
    app::{Route, Stubs},
    components::{DFTable, DefaultHead, TopBar, StockIcon},
};

#[component]
pub fn StatsPage(cx: Scope, index: usize) -> Element {
    let game: Game = use_shared_state::<Stubs>(cx).unwrap().read().0[*index]
        .clone()
        .into();
    let p1 = game.players[0].clone();
    let p2 = game.players[1].clone();
    // let game = use_ref(cx, || parse(path.to_str().unwrap(), false).pop().unwrap());
    let port: &UseRef<Port> = use_ref(cx, || Port::P1);
    let stat_type: &UseRef<StatType> = use_ref(cx, || StatType::Defense);
    let navigator = use_navigator(cx);

    render! (
        DefaultHead {}

        body { class: "dark mask", visibility: "hidden",
            header {
                nav {
                    button { class: "circle transparent", onclick: |_evt| { navigator.push(Route::Browse {  });}, i { "arrow_back" }, }
                    div { class: "max" }
                    label { class: "radio max",
                        input {
                            r#type: "radio",
                            id: "player_1",
                            name: "players",
                            value: "p1",
                            checked: "true",
                            onclick: move |_evt| {
                                let mut p = port.write();
                                *p = p1.port;
                            }
                        }
                        span {"{p1.connect_code.clone().unwrap_or(Port::P1.to_string())} "}
                        StockIcon { character: (p1.character, p1.costume) }
                    }

                    label { class: "radio max",
                        input {
                            r#type: "radio",
                            id: "player_2",
                            name: "players",
                            value: "p2",
                            onclick: move |_evt| {
                                let mut p = port.write();
                                *p = p2.port;
                            }
                        }
                        span {"{p2.connect_code.clone().unwrap_or(Port::P2.to_string())} "}
                        StockIcon { character: (p2.character, p2.costume) }
                    }

                    div { class: "field suffix border max",
                        select {
                            name: "Stat Type",
                            id: "stat_type",
                            onchange: |evt| {
                                let mut st = stat_type.write();
                                *st = StatType::from_str(&evt.value).unwrap();
                            },
                            option { value: "{StatType::Defense}", "Defense" }
                            option { value: "{StatType::Tech}", "Tech" }
                            option { value: "{StatType::Wavedash}", "Wavedash" }
                            option { value: "{StatType::LCancel}", "LCancel" }
                            option { value: "{StatType::Input}", "Input" }
                            option { value: "{StatType::Item}", "Item" }
                        }
                        i { "arrow_drop_down" }
                    }
                    div { class: "max" }
                }
            }

            main { class: "max", height: "calc(100dvh - 64px)",
                div { display: "flex", flex_direction: "column", max_height: "calc(100dvh - 64px)",
                    p {
                        padding_bottom: 0,
                        margin_bottom: 0,
                        "Game Info:"
                    }
                    figure {
                        overflow: "auto",
                        flex: "1 0 auto",
                    DFTable{ dataframe: game.summarize() }
                    }

                    p {
                        padding_bottom: 0,
                        margin_bottom: 0,
                        "Summary:"
                    }
                    div {
                        class: "scroll",
                        bottom: 0,
                        overflow: "auto",
                        flex: "1 0 auto",
                        margin: 0,
                        padding: 0,
                        // max_height: "50vh",
                        DFTable{ dataframe: game
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
                                DFTable{ dataframe: game
                                    .player_by_port(*port.read())
                                    .unwrap()
                                    .stats
                                    .get(*stat_type.read())
                                    .unwrap()
                                }
                            }
                        )
                    }
                }
            }
        }
    )
}
