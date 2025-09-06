use dioxus::prelude::*;

use crate::{Footer, Header};

#[component]
pub fn Links() -> Element {
    rsx! {
        Header {}
        div {
            class: "links_list",
            h1 {"Links"}
            div {class: "links_link", Link {to: "https://codeberg.org/Tahinli", class: "links_link", "Codeberg"} }
            div {class: "links_link", Link {to: "https://github.com/Tahinli", class: "links_link", "GitHub"} }
            div {class: "links_link", Link {to: "https://linkedin.com/in/ahmetkaangumus", class: "links_link", "LinkedIn"} }
        }
        Footer {}
    }
}
