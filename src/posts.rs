pub mod open_letters;
pub mod writings;

use crate::{Footer, Header, Route};
use dioxus::prelude::*;

#[component]
pub fn Posts() -> Element {
    rsx! {
        Header {}
        div {
            class: "posts_list",
            h1 {"Posts"}
            div { class: "post_link", Link {to: Route::Writings {  }, class: "post_link", "Writings"} }
            div { class: "post_link", Link {to: Route::OpenLetters {  }, class: "post_link", "Open Letters"} }
        }
        Footer {}
    }
}
