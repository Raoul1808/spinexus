#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChartData {
    pub id: i64,
    pub title: String,
    pub artist: String,
    pub charter: String,
    pub uploader: String,
}

#[inline_props]
fn ChartListing(cx: Scope, chart: ChartData) -> Element {
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
                    "Uploaded by {uploader}"
                }
            }
        }
    }
}

fn App(cx: Scope) -> Element {
    render! {
        ChartListing {
            chart: ChartData {
                id: 727,
                title: "WYSI - When You See It".to_string(),
                artist: "Camellia".to_string(),
                charter: "The charter who sees it".to_string(),
                uploader: "The uploader who sees it".to_string(),
            }
        }
    }
}

fn main() {
    dioxus_desktop::launch(App);
}
