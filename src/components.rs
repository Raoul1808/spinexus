use dioxus::prelude::*;
use dioxus_router::components::Link;

use crate::route::Route;
use crate::models::*;

#[derive(PartialEq)]
pub enum ChartListingMode {
    New,
    Updated,
    HotMonth,
    HotWeek,
    User(i32),
}

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

pub fn ShowLoading(cx: Scope) -> Element {
    render! {
        div {
            class: "center",
            "Loading..."
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
            ChartListing { mode: ChartListingMode::User(*id) }
        }
    }
}

#[inline_props]
pub fn ChartFullDisplay<'a>(cx: Scope, chart: &'a FullChart) -> Element {
    let FullChart {
        title,
        artist,
        charter,
        uploader,
        cover,
        ..
    } = chart;

    let zip = &chart.paths.zip;

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
            UserShortDisplay { id: *uploader }
            button {
                onclick: move |event| { println!("Event {event:?} with zip {zip}"); },
                "Download"
            }
        }
    }
}

#[inline_props]
fn ChartShortDisplay<'a>(cx: Scope, chart: &'a PartialChart) -> Element {
    let PartialChart {
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

#[inline_props]
pub fn ChartListing(cx: Scope, mode: ChartListingMode) -> Element {
    let charts = match mode {
        ChartListingMode::New => use_future(cx, (), |_| get_new_charts(0)),
        ChartListingMode::Updated => use_future(cx, (), |_| get_updated_charts(0)),
        ChartListingMode::HotWeek => use_future(cx, (), |_| get_weekly_hot_charts(0)),
        ChartListingMode::HotMonth => use_future(cx, (), |_| get_monthly_hot_charts(0)),
        ChartListingMode::User(id) => use_future(cx, (), |_| get_charts_for_user(*id)),
    };

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
                ShowLoading {}
            }
        }
    }
}
