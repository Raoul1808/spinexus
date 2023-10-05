#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};
use dioxus_router::prelude::*;

mod models;

use models::{Chart, User};
use models::get_chart;
use models::get_hot_charts;
use models::get_user;

#[derive(Routable, PartialEq, Debug, Clone)]
enum Route {
    #[route("/")]
    Index {},
    #[route("/chart/:id")]
    Chart { id: i32 },
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

fn Index(cx: Scope) -> Element {
    render! {
        div {
            "This is the index page!"
        }
        ChartListing {}
    }
}

#[inline_props]
fn NotFound(cx: Scope, route: Vec<String>) -> Element {
    render! {
        div {
            "The page {route.join(\"/\")} doesn't exist."
        }
    }
}

#[inline_props]
fn Chart(cx: Scope, id: i32) -> Element {
    let chart = use_future(cx, (), |_| get_chart(*id));
    match chart.value() {
        Some(Ok(chart)) => {
            render! {
                div {
                    Link {
                        to: Route::Index {},
                        "<-- Go back home"
                    }
                }
                ChartFullDisplay { chart: chart }
            }
        }
        Some(Err(err)) => {
            render! {"An error occurred while fetching chart: {err}"}
        }
        None => {
            render! {"API stuff loading thing idk"}
        }
    }
}

#[inline_props]
fn ShowUploader(cx: Scope, id: i32) -> Element {
    let user = use_future(cx, (), |_| get_user(*id));
    match user.value() {
        Some(Ok(user)) => {
            let User {
                username,
                avatar,
                ..
            } = user;
            render! {
                img {
                    width: "64px",
                    src: "{avatar}"
                }
                div {
                    "{username}"
                }
            }
        }
        Some(Err(err)) => {
            render! {"Err {err}"}
        }
        None => {
            render! {"wut"}
        }
    }
}

#[inline_props]
fn ChartFullDisplay<'a>(cx: Scope, chart: &'a Chart) -> Element {
    let Chart {
        title,
        artist,
        charter,
        uploader,
        cover,
        ..
    } = chart;

    render! {
        div {
            img {
                src: "{cover}"
            }
        }
        div {
            div {
                font_size: 32,
                "{title}"
            }
            div {
                color: "gray",
                "{artist}"
            }
            div {
                "Charted by {charter}"
            }
            if let Some(uploader) = uploader {
                rsx! {
                    ShowUploader { id: *uploader }
                }
            }
        }
    }
}

#[inline_props]
fn ChartShortDisplay<'a>(cx: Scope, chart: &'a Chart) -> Element {
    let Chart {
        id,
        title,
        artist,
        charter,
        uploader,
        ..
    } = chart;

    render! {
        div {
            padding: "0.5em",
            position: "relative",
            div {
                font_size: "1.5em",
                Link {
                    to: Route::Chart { id: chart.id },
                    "{title} (#{id})"
                }
            }
            div {
                display: "flex",
                flex_direction: "row",
                color: "gray",
                div {
                    "By {artist}"
                }
                div {
                    padding_left: "0.5rem",
                    "Charted by {charter}"
                }
                if let Some(uploader) = uploader {
                    rsx! {
                        div {
                            padding_left: "0.5rem",
                            "Uploaded by #{uploader}"
                        }
                    }
                }
            }
        }
    }
}

fn ChartListing(cx: Scope) -> Element {
    let charts = use_future(cx, (), |_| get_hot_charts());
    match charts.value() {
        Some(Ok(charts)) => {
            render! {
                for chart in &charts {
                    ChartShortDisplay { chart: chart }
                }
            }
        }
        Some(Err(err)) => {
            render! {"An error occurred while fetching chart: {err}"}
        }
        None => {
            render! {"API stuff loading thing idk"}
        }
    }
}

fn App(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
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
