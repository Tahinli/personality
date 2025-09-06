use dioxus::prelude::*;

use crate::{Footer, Header};

#[component]
pub fn Links() -> Element {
    rsx! {
        Header {}
        div {
            class: "links_list",
            h1 {"Links"}
            div {class: "links_link", Link {to: "https://linkedin/in/ahmetkaangumus", class: "links_link", "LinkedIn"} }
            div {class: "links_link", Link {to: "https://github.com/Tahinli", class: "links_link", "GitHub"} }
            div {class: "links_link", Link {to: "https://source.tahinli.com/Tahinli", class: "links_link", "My Personal Git Website"} }
        }
        Footer {}
    }
}
