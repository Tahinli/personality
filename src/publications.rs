use dioxus::prelude::*;

use crate::{Footer, Header};

#[component]
pub fn Publications() -> Element {
    rsx! {
        Header {}
        div {
            class: "publications_list",
            h1 {"Publications"}
        }
        Footer {}
    }
}

#[component]
pub fn Publication(id: i32) -> Element {
    let x = match id {
        1 => {
            rsx! {
            }
        }
        _ => {
            rsx! {
                h1 {"You're not supposed to be here"}
            }
        }
    };
    rsx! {
        Header {}
        {x}
        Footer {}
    }
}
