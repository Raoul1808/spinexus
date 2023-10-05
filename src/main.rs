#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod models;

use models::Chart;
use models::get_hot_charts;

#[derive(Routable, PartialEq, Debug, Clone)]
enum Route {
    #[route("/")]
    Index {},
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
fn ChartDisplay<'a>(cx: Scope, chart: &'a Chart) -> Element {
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
                    to: Route::NotFound { route: (vec!["hello".into(), "world".into()]) },
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
                    ChartDisplay { chart: chart }
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
    dioxus_desktop::launch(App);
}
