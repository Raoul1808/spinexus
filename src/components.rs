use dioxus::prelude::*;
use dioxus_router::components::Link;

use crate::route::Route;
use crate::models::*;

pub fn BackHome(cx: Scope) -> Element {
    render! {
        div {
            Link {
                to: Route::Index {},
                "Home"
            }
        }
    }
}

#[inline_props]
fn UserShortDisplay(cx: Scope, id: i32) -> Element {
    let user = use_future(cx, (), |_| get_user(*id));
    match user.value() {
        Some(Ok(user)) => {
            let User {
                username,
                avatar,
                ..
            } = user;
            render! {
                Link {
                    to: Route::User { id: *id },
                    div {
                        class: "user-short-view",
                        img {
                            width: "64px",
                            src: "{avatar}"
                        }
                        div {
                            "{username}"
                        }
                    }
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
pub fn UserFullDisplay<'a>(cx: Scope, user: &'a User) -> Element {
    let User {
        id,
        username,
        avatar,
        ..
    } = user;

    render! {
        div {
            img {
                src: "{avatar}"
            }
            div {
                "{username}"
            }
        }
        div {
            h2 {
                "Charts uploaded"
            }
            ChartUserDisplay {id: *id}
        }
    }
}

#[inline_props]
pub fn ChartFullDisplay<'a>(cx: Scope, chart: &'a Chart) -> Element {
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
                    UserShortDisplay { id: *uploader }
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

pub fn ChartNewListing(cx: Scope) -> Element {
    let charts = use_future(cx, (), |_| get_new_charts(0));
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

pub fn ChartUpdatedListing(cx: Scope) -> Element {
    let charts = use_future(cx, (), |_| get_updated_charts(0));
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

pub fn ChartHotWeekListing(cx: Scope) -> Element {
    let charts = use_future(cx, (), |_| get_weekly_hot_charts(0));
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

pub fn ChartHotMonthListing(cx: Scope) -> Element {
    let charts = use_future(cx, (), |_| get_monthly_hot_charts(0));
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

#[inline_props]
fn ChartUserDisplay(cx: Scope, id: i32) -> Element {
    let charts = use_future(cx, {}, |_| get_charts_for_user(*id));
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
