use dioxus::prelude::*;
use dioxus_router::prelude::*;

use std::str::FromStr;
use polars::prelude::*;
use slpprocess::{parse, player::Player, stats::StatType, Game};
use ssbm_utils::enums::Port;

use crate::app::Route;

#[component]
pub fn TopBar<'a>(
    cx: Scope<'a>,
    players: [Arc<Player>; 2],
    port: &'a UseRef<Port>,
    stat_type: &'a UseRef<StatType>,
) -> Element {
    cx.render(rsx!(
        nav { gap: "20px",
            Link { to: Route::Browse {}, button { flex: "0 0 auto", r#type: "button", "Back" } }

            div {
                input {
                    r#type: "radio",
                    id: "player_1",
                    name: "players",
                    value: "p1",
                    checked: "true",
                    onclick: |_evt| {
                        let mut p = port.write();
                        *p = players[0].port;
                    }
                }
                label { r#for: "player_1",
                    "{players[0].connect_code.clone().unwrap_or(Port::P1.to_string())} | {players[0].character.to_string()}"
                }
            }

            div {
                input {
                    r#type: "radio",
                    id: "player_2",
                    name: "players",
                    value: "p2",
                    onclick: |_evt| {
                        let mut p = port.write();
                        *p = players[1].port;
                    }
                }
                label { r#for: "player_2",
                    "{players[1].connect_code.clone().unwrap_or(Port::P2.to_string())} | {players[1].character.to_string()}"
                }
            }

            select {
                name: "Stat Type",
                id: "stat_type",
                max_width: "33%",
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
        }
    ))
}
