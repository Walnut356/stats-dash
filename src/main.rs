#![allow(non_snake_case)]
#![cfg_attr(feature = "bundle", windows_subsystem = "windows")]
use dioxus_desktop::Config;
use dioxus_router::prelude::*;

use dioxus::prelude::*;
use log::LevelFilter;

use slpprocess::parse;
use polars::prelude::*;

use stats_dash::*;

fn main() {
    std::env::set_var("POLARS_FMT_STR_LEN", "200");    // maximum number of characters printed per string value.
    std::env::set_var("POLARS_FMT_TABLE_CELL_LIST_LEN", "-1");
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus_desktop::launch_cfg(app, Config::new().with_data_directory("./resources").with_root_name("StatsDash"));
}
