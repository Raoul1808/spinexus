#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};
use dioxus_router::prelude::*;

mod components;
mod models;
mod route;

fn App(cx: Scope) -> Element {
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
