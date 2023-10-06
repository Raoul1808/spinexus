#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};
use dioxus_router::prelude::*;

mod models;
use models::*;

#[derive(Routable, PartialEq, Debug, Clone)]
enum Route {
    #[route("/")]
    Index {},
    #[nest("/charts")]
        #[route("/updated")]
        UpdatedCharts {},
        #[route("/month")]
        HotMonthCharts {},
        #[route("/week")]
        HotWeekCharts {},
    #[end_nest]
    #[route("/chart/:id")]
    Chart { id: i32 },
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

fn Index(cx: Scope) -> Element {
    render! {
        BackHome {}
        div {
            "This is the index page!"
        }
        ul {
            li {
                Link {
                    to: Route::UpdatedCharts {},
                    "Updated"
                }
            }
            li {
                Link {
                    to: Route::HotMonthCharts {},
                    "Hot this month"
                }
            }
            li {
                Link {
                    to: Route::HotWeekCharts {},
                    "Hot this week"
                }
            }
        }
    }
}

fn UpdatedCharts(cx: Scope) -> Element {
    render! {
        BackHome {}
        h1 {
            "Last updated charts"
        }
        ChartUpdatedListing {}
    }
}

fn HotMonthCharts(cx: Scope) -> Element {
    render! {
        BackHome {}
        h1 {
            "Hot this month"
        }
        ChartHotMonthListing {}
    }
}

fn HotWeekCharts(cx: Scope) -> Element {
    render! {
        BackHome {}
        h1 {
            "Hot this week"
        }
        ChartHotWeekListing {}
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
                BackHome {}
                ChartFullDisplay { chart: chart }
            }
        }
        Some(Err(err)) => {
            render! {
                BackHome {}
                "An error occurred while fetching chart: {err}"
            }
        }
        None => {
            render! {
                BackHome {}
                "API stuff loading thing idk"
            }
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
        title,
        artist,
        charter,
        cover,
        ..
    } = chart;

    render! {
        Link {
            class: "no-underline",
            to: Route::Chart { id: chart.id },
            div {
                class: "chart-short-view",
                img {
                    src: "{cover}"
                }
                div {
                    div {
                        font_size: "1.5em",
                        "{title}"
                    }
                    div {
                        "{artist} • Charted by {charter}"
                    }
                }
            }
        }
    }
}

fn ChartUpdatedListing(cx: Scope) -> Element {
    let charts = use_future(cx, (), |_| get_updated_charts());
    match charts.value() {
        Some(Ok(charts)) => {
            render! {
                div {
                    class: "chart-list-view",
                    for chart in &charts {
                        ChartShortDisplay { chart: chart }
                    }
                }
            }
        }
        Some(Err(err)) => {
            render! {
                "An error occurred while fetching charts: {err}"
            }
        }
        None => {
            render! {
                "API stuff loading thing idk"
            }
        }
    }
}

fn ChartHotWeekListing(cx: Scope) -> Element {
    let charts = use_future(cx, (), |_| get_weekly_hot_charts());
    match charts.value() {
        Some(Ok(charts)) => {
            render! {
                div {
                    class: "chart-list-view",
                    for chart in &charts {
                        ChartShortDisplay { chart: chart }
                    }
                }
            }
        }
        Some(Err(err)) => {
            render! {
                "An error occurred while fetching charts: {err}"
            }
        }
        None => {
            render! {
                "API stuff loading thing idk"
            }
        }
    }
}

fn ChartHotMonthListing(cx: Scope) -> Element {
    let charts = use_future(cx, (), |_| get_monthly_hot_charts());
    match charts.value() {
        Some(Ok(charts)) => {
            render! {
                div {
                    class: "chart-list-view",
                    for chart in &charts {
                        ChartShortDisplay { chart: chart }
                    }
                }
            }
        }
        Some(Err(err)) => {
            render! {"An error occurred while fetching charts: {err}"}
        }
        None => {
            render! {"API stuff loading thing idk"}
        }
    }
}

fn BackHome(cx: Scope) -> Element {
    render! {
        div {
            Link {
                to: Route::Index {},
                "Home"
            }
        }
    }
}

fn App(cx: Scope) -> Element {
    render! {
        style { include_str!("../style.css") }
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
