#![allow(non_snake_case)]

mod app_config;
mod components;
mod download;
mod models;
mod route;

use dioxus::prelude::*;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};
use dioxus_router::prelude::*;

use app_config::AppConfig;

fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || AppConfig::load().unwrap_or_default());
    render! {
        style { include_str!("../style.css") }
        Router::<route::Route> {}
    }
}

fn main() {
    let window = WindowBuilder::new()
        .with_title("Spinexus")
        .with_resizable(false)
        .with_maximizable(false)
        .with_inner_size(LogicalSize::new(1280, 720));
    let config = Config::new()
        .with_window(window);
    dioxus_desktop::launch_cfg(App, config);
}
