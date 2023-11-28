use dioxus::prelude::*;
use dioxus_router::components::{Link, GoBackButton, GoForwardButton};

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
    SearchChart(String, i32),
}

pub fn HeaderButtons(cx: Scope) -> Element {
    render! {
        div {
            Link {
                class: "btn btn-outline-blue m-3",
                to: Route::Index {},
                "<< Home"
            }
            GoBackButton {
                div {
                    class: "btn btn-outline-blue m-3",
                    "< Back"
                }
            }
            GoForwardButton {
                div {
                    class: "btn btn-outline-blue m-3",
                    "Forward >"
                }
            }
        }
    }
}

pub fn ShowLoading(cx: Scope) -> Element {
    render! {
        div {
            class: "text-center",
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
                div {
                    class: "relative rounded bg-gray-200 no-underline m-1 max-w-sm overflow-hidden",
                    Link {
                        to: Route::User { id: *id },
                        div {
                            class: "flex items-center",
                            img {
                                class: "absolute aspect-square rounded-full shadow-lg w-28 h-28 -left-6",
                                src: "{avatar}"
                            }
                            div {
                                class: "min-w-0 py-5 pl-28",
                                p {
                                    class: "font-bold text-gray-900",
                                    "{username}"
                                }
                                p {
                                    class: "text-sm font-medium text-slate-700",
                                    "Uploader"
                                }
                            }
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
        div {
            class: "rounded-xl bg-gray-200 no-underline m-1",
            Link {
                to: Route::Chart { id: chart.id },
                div {
                    class: "flex items-center space-x-4 p-4",
                    img {
                        class: "aspect-square rounded-lg object-left",
                        width: 88,
                        src: "{cover}"
                    }
                    div {
                        class: "flex-auto space-y-1 font-semibold",
                        p {
                            class: "text-2xl",
                            "{title}"
                        }
                        p {
                            class: "text-gray-800",
                            "{artist}"
                        }
                        p {
                            class: "text-gray-600",
                            "Charted by {charter}"
                        }
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
        ChartListingMode::SearchChart(query, _) => use_future(cx, (query,), |_| search_chart(query.clone())),
    };

    match charts.value() {
        Some(Ok(charts)) => {
            render! {
                div {
                    class: "grid grid-cols-3",
                    if let ChartListingMode::SearchChart(query, page) = mode {
                        let cur_chart = (page * 12) as usize;
                        let max_chart = std::cmp::min(((page + 1) * 12) as usize, charts.len());
                        println!("{}, {}, {}, {:#?}", page, cur_chart, max_chart, charts);
                        let charts = charts.get(cur_chart..max_chart);
                        if let Some(charts) = charts {
                            if charts.len() <= 0 {
                                rsx! {
                                    "No charts found for {query}"
                                }
                            }
                            else {
                                rsx! {
                                    for chart in &charts {
                                        ChartShortDisplay { chart: chart }
                                    }
                                }
                            }
                        }
                        else {
                            rsx! {
                                "Search out of bounds"
                            }
                        }
                    }
                    else {
                        rsx! {
                            for chart in &charts {
                                ChartShortDisplay { chart: chart }
                            }
                        }
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
