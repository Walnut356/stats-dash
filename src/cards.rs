use dioxus::prelude::*;
use dioxus_router::components::Link;
use slpprocess::game::GameStub;
use time::UtcOffset;

use crate::Route;

#[inline_props]
pub fn Card(cx: Scope, stub: GameStub) -> Element {
    let date = stub.metadata.date;
    let time = date.to_offset(UtcOffset::current_local_offset().unwrap()).time();
    let date_str = format!(
        "{} {} {}, {} at {:0>2}:{:0>2}:{:0>2}",
        date.weekday(),
        date.month(),
        date.day(),
        date.year(),
        time.hour(),
        time.minute(),
        time.second(),
    );

    let p1 = &stub.players[0];
    let p2 = &stub.players[1];
    cx.render(rsx! {
        button {
            background_color: "#11191f",
            Link {
                to: Route::StatsPage {
                    path: stub.path.clone(),
                },
                header {

                    div {
                        display: "flex",
                        justify_content: "space-between",
                        p {
                            color: "#fff",
                            font_size: "75%",
                            "{date_str}"
                        }
                        p {color: "#fff", "{stub.metadata.stage.to_string()}"}
                        p {
                            color: "#fff",
                            font_size: "75%",
                            "{stub.path.file_name().unwrap().to_str().unwrap()}"
                        }
                    }
                }
                div {
                    display: "flex",
                    gap: "20em",
                    justify_content: "space-evenly",
                    p {color: "#fff", "{stub.players[0].display_name.clone().unwrap_or_default()} ({stub.players[0].connect_code.clone().unwrap_or_default()}) | {stub.players[0].character.to_string()}" }
                    p {color: "#fff", "{stub.players[1].display_name.clone().unwrap_or_default()} ({stub.players[1].connect_code.clone().unwrap_or_default()}) | {stub.players[1].character.to_string()}"}
                }
            }
        }
    })
}
