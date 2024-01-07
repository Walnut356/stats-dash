use dioxus::prelude::*;
use dioxus_router::prelude::*;

use slpprocess::GameStub;
use ssbm_utils::enums::Character;
use crate::pages::{
    StatsPage,
    Browse
};

pub struct WorkingDir(pub String);
pub struct Stubs(pub Vec<GameStub>);
pub struct Sets(pub Vec<(usize, usize)>);

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Filters {
    pub code: String,
    pub display_name: String,
    pub character: Option<Character>,
}

pub fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, || WorkingDir("".to_string()));
    use_shared_state_provider(cx, || Stubs(Vec::<GameStub>::new()));
    use_shared_state_provider(cx, || Sets(Vec::new()));
    render! { Router::<Route> {} }
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[route("/")]
    Browse {},
    #[route("/stats")]
    StatsPage { index: usize },
}