use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn DefaultHead(cx: Scope) -> Element {
    render!(
        head {
            link { rel: "stylesheet", href: "./dist/beer.min.css" }
            script { r#type: "module", src: "./dist/beer.min.js" }
            script { r#type: "module", src: "./dist/material-dynamic-colors.min.js" }
            script { src: "maintainscroll.js" }
        }
    )
}
