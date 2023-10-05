#![allow(non_snake_case)]

use dioxus::prelude::*;

mod models;

use models::Chart;
use models::get_chart;

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
                "{title} (#{id})"
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
                div {
                    padding_left: "0.5rem",
                    "Uploaded by #{uploader}"
                }
            }
        }
    }
}

fn ChartListing(cx: Scope) -> Element {
    let chart = use_future(cx, (), |_| get_chart(1116));

    match chart.value() {
        Some(Ok(chart)) => {
            render! {
                div {
                    ChartDisplay { chart: &chart }
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
        div {
            ChartListing {}
        }
    }
}

fn main() {
    dioxus_desktop::launch(App);
}
