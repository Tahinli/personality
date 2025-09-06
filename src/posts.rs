use dioxus::prelude::*;

use crate::{Footer, Header};

#[component]
pub fn Posts() -> Element {
    rsx! {
        Header {}
        div {
            class: "posts_list",
            h1 {"Posts"}
        }
        Footer {}
    }
}

#[component]
pub fn Post(id: i32) -> Element {
    let x = match id {
        1 => {
            rsx! {}
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
