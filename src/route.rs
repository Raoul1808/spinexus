use dioxus::prelude::*;
use dioxus_router::prelude::*;
use directories::UserDirs;
use rfd::FileDialog;

use crate::app_config::AppConfig;
use crate::components::*;
use crate::models::{get_chart, get_user};

#[derive(Routable, PartialEq, Debug, Clone)]
pub enum Route {
    #[route("/")]
    Index {},
    #[nest("/charts")]
        #[route("/new")]
        NewCharts {},
        #[route("/updated")]
        UpdatedCharts {},
        #[route("/month")]
        HotMonthCharts {},
        #[route("/week")]
        HotWeekCharts {},
        #[route("/search/:query")]
        SearchCharts { query: String },
    #[end_nest]
    #[route("/chart/:id")]
    Chart { id: i32 },
    #[route("/user/:id")]
    User { id: i32 },
    #[route("/settings")]
    AppSettings {},
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

fn Index(cx: Scope) -> Element {
    let query = use_state(cx, || "".to_string());
    let navigator = use_navigator(cx);
    if !query.is_empty() {
        navigator.push(Route::SearchCharts { query: query.to_string() });
    }
    render! {
        HeaderButtons {}
        div {
            form {
                onsubmit: move |event| {
                    let search_query = match event.values.get("search-query") {
                        Some(q) => &q[0],
                        None => return,
                    };
                    query.with_mut(|q| {
                        *q = search_query.to_string()
                    });
                },
                input {
                    name: "search-query",
                    r#type: "text",
                }
                input {
                    r#type: "submit",
                }
            }
        }
        div {
            class: "text-5xl mx-auto my-3",
            "This is the index page!"
        }
        Link {
            class: "btn btn-blue m-1",
            to: Route::NewCharts {},
            "New"
        }
        Link {
            class: "btn btn-blue m-1",
            to: Route::UpdatedCharts {},
            "Updated"
        }
        Link {
            class: "btn btn-blue m-1",
            to: Route::HotMonthCharts {},
            "Hot this month"
        }
        Link {
            class: "btn btn-blue m-1",
            to: Route::HotWeekCharts {},
            "Hot this week"
        }
        Link {
            class: "btn btn-blue m-1",
            to: Route::AppSettings {},
            "Application Settings"
        }
    }
}

fn NewCharts(cx: Scope) -> Element {
    let mut page = use_state(cx, || 0);
    render! {
        HeaderButtons{}
        h1 {
            "Newest charts - Page {page}"
        }
        if **page > 0 {
            rsx! {
                button { class: "btn btn-blue", onclick: move |_| page -= 1, "Previous Page" },
            }
        }
        if **page < 6 {
            rsx! {
                button { class: "btn btn-blue", onclick: move |_| page += 1, "Next Page" },
            }
        }
        ChartListing { mode: ChartListingMode::New(**page) }
    }
}

fn UpdatedCharts(cx: Scope) -> Element {
    let mut page = use_state(cx, || 0);
    render! {
        HeaderButtons {}
        h1 {
            "Last updated charts"
        }
        if **page > 0 {
            rsx! {
                button { class: "btn btn-blue", onclick: move |_| page -= 1, "Previous Page" },
            }
        }
        if **page < 6 {
            rsx! {
                button { class: "btn btn-blue", onclick: move |_| page += 1, "Next Page" },
            }
        }
        ChartListing { mode: ChartListingMode::Updated(**page) }
    }
}

fn HotMonthCharts(cx: Scope) -> Element {
    let mut page = use_state(cx, || 0);
    render! {
        HeaderButtons {}
        h1 {
            "Hot this month"
        }
        if **page > 0 {
            rsx! {
                button { class: "btn btn-blue", onclick: move |_| page -= 1, "Previous Page" },
            }
        }
        if **page < 6 {
            rsx! {
                button { class: "btn btn-blue", onclick: move |_| page += 1, "Next Page" },
            }
        }
        ChartListing { mode: ChartListingMode::HotMonth(**page) }
    }
}

fn HotWeekCharts(cx: Scope) -> Element {
    let mut page = use_state(cx, || 0);
    render! {
        HeaderButtons {}
        h1 {
            "Hot this week"
        }
        if **page > 0 {
            rsx! {
                button { class: "btn btn-blue", onclick: move |_| page -= 1, "Previous Page" },
            }
        }
        if **page < 6 {
            rsx! {
                button { class: "btn btn-blue", onclick: move |_| page += 1, "Next Page" },
            }
        }
        ChartListing { mode: ChartListingMode::HotWeek(**page) }
    }
}

#[inline_props]
fn SearchCharts(cx: Scope, query: String) -> Element {
    let mut page = use_state(cx, || 0);
    render! {
        HeaderButtons {},
        h1 {
            "Search results for {query}"
        }
        if **page > 0 {
            rsx! {
                button { class: "btn btn-blue", onclick: move |_| page -= 1, "Previous Page" },
            }
        }
        if **page < 6 {
            rsx! {
                button { class: "btn btn-blue", onclick: move |_| page += 1, "Next Page" },
            }
        }
        ChartListing { mode: ChartListingMode::SearchChart(query.to_string(), **page) }
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
                HeaderButtons {}
                ChartFullDisplay { chart: chart }
            }
        }
        Some(Err(err)) => {
            render! {
                HeaderButtons {}
                "An error occurred while fetching chart: {err}"
            }
        }
        None => {
            render! {
                HeaderButtons {}
                ShowLoading {}
            }
        }
    }
}

#[inline_props]
fn User(cx: Scope, id: i32) -> Element {
    let user = use_future(cx, (), |_| get_user(*id));
    match user.value() {
        Some(Ok(user)) => {
            render! {
                HeaderButtons{}
                UserFullDisplay { user: user }
            }
        }
        Some(Err(err)) => {
            render! {
                HeaderButtons {}
                "An error occurred while fetching user: {err}"
            }
        }
        None => {
            render! {
                HeaderButtons {}
                ShowLoading {}
            }
        }
    }
}

fn AppSettings(cx: Scope) -> Element {
    let config = use_shared_state::<AppConfig>(cx).unwrap();
    let customs_path = &config.read().customs_path;

    render! {
        HeaderButtons {}
        div {
            span {
                "Current path: {customs_path}"
            }
            button {
                onclick: move |_event| {
                    let user_dir = UserDirs::new().unwrap();
                    let home_dir = user_dir.home_dir();
                    let folder = FileDialog::new()
                        .set_directory(home_dir)
                        .pick_folder();
                    if let Some(folder) = folder {
                        config.write().customs_path = folder.as_path().display().to_string();
                        let _ = config.write().save();
                    }
                },
                "Browse"
            }
        }
    }
}
