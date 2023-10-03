#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    render! {
        TestComponent {

        }
    }
}

fn TestComponent(cx: Scope) -> Element {
    let name = "Mew";
    let time = chrono::Utc::now();

    render! {
        div {
            padding: "0.5em",
            position: "relative",
            "Hello, {name}! It is currently {time}"
        }
    }
}
