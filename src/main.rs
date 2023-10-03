#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChartData {
    pub id: i32,
    pub title: String,
    pub artist: String,
    pub charter: String,
    pub uploader: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChartRequestData {
    pub version: i32,
    pub status: i32,
    pub data: ChartData,
}

async fn get_chart(id: i64) -> Result<ChartData, reqwest::Error> {
    let url = format!("https://spinsha.re/api/song/{}", id);
    let chart = reqwest::get(&url).await?.json::<ChartRequestData>().await?;
    Ok(chart.data)
}

#[inline_props]
fn ChartDisplay(cx: Scope, chart: ChartData) -> Element {
    let ChartData {
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
                    ChartDisplay { chart: chart.clone() }
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
