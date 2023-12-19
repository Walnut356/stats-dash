#![allow(non_snake_case)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use dioxus_desktop::{Config, WindowBuilder, tao::window::Icon};
use dioxus_router::prelude::*;

use dioxus::prelude::*;

use log::LevelFilter;

use polars::prelude::*;
use slpprocess::parse;

use stats_dash::*;

fn main() {
    std::env::set_var("POLARS_FMT_STR_LEN", "200"); // maximum number of characters printed per string value.
    std::env::set_var("POLARS_FMT_TABLE_CELL_LIST_LEN", "-1");
    // Init debug

    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    dioxus_desktop::launch_cfg(
        app,
        Config::new()
            .with_resource_directory("./dist/")
            .with_data_directory("./dist/")
            .with_window(WindowBuilder::new().with_title("Stats Dash"))
            .with_background_color((17, 25, 31, 1))
    );
}
