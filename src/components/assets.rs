use dioxus::prelude::*;
use ssbm_utils::prelude::*;

#[component]
pub fn StockIcon(cx: Scope, character: (Character, Costume), opacity: Option<f64>) -> Element {
    render! (
        img {
            height: "24px",
            width: "24px",
            opacity: "{opacity.unwrap_or(1.0)}",
            src: "./dist/Characters/{character.0.to_string()}/{character.1.to_string()}.png"
        }
    )
}

#[component]
pub fn StageImg(cx: Scope, stage: StageID) -> Element {
    render!(
        img {
            height: "48px",
            width: "64px",
            src: "./dist/Stages/{stage.to_string()}.png"
        }
    )
}