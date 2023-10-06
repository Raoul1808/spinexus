use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::components::*;
use crate::models::get_chart;

#[derive(Routable, PartialEq, Debug, Clone)]
pub enum Route {
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
