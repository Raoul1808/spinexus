use dioxus::prelude::*;
use dioxus_router::components::Link;

use std::path::PathBuf;

use crate::app_config::AppConfig;
use crate::route::Route;
use crate::models::*;
use crate::download::download_and_extract_zip;

#[derive(PartialEq, Debug)]
pub enum ChartListingMode {
    New(i32),
    Updated(i32),
    HotMonth(i32),
    HotWeek(i32),
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
    let app_config = use_shared_state::<AppConfig>(cx).unwrap();

    let FullChart {
        title,
        artist,
        charter,
        uploader,
        cover,
        file_reference,
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
                onclick: move |_| {
                    let zip = zip.clone();
                    let destination = app_config.read().customs_path.clone();
                    let file_reference = file_reference.clone();
                    let proj_dir = directories::ProjectDirs::from("rs", "", "spinexus").unwrap();
                    let cache_dir = PathBuf::from(proj_dir.cache_dir()).to_str().unwrap().to_string();
                    async {
                        println!("Downloading file {zip}");
                        match download_and_extract_zip(zip, cache_dir, destination, file_reference).await {
                            Ok(_) => println!("Download complete!"),
                            Err(e) => println!("Error {e}"),
                        };
                    }
                },
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
        ChartListingMode::New(page) => use_future(cx, (page,), |_| get_new_charts(*page)),
        ChartListingMode::Updated(page) => use_future(cx, (page,), |_| get_updated_charts(*page)),
        ChartListingMode::HotWeek(page) => use_future(cx, (page,), |_| get_weekly_hot_charts(*page)),
        ChartListingMode::HotMonth(page) => use_future(cx, (page,), |_| get_monthly_hot_charts(*page)),
        ChartListingMode::User(id) => use_future(cx, (id,), |_| get_charts_for_user(*id)),
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
