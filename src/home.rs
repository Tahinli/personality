use dioxus::prelude::*;

use crate::{Footer, Header, Route};

#[component]
pub fn Home() -> Element {
    rsx! {
        Header {}
        div { id: "content",
            Outlet::<Route> {}
        }
        div {
            class: "main",
            h1 {
                id: "name",
                "Ahmet Kaan Gümüş"}
            h4 {
                id: "title",
                "Software Engineer"}

            div {
                id: "main_about",
                "
               Hi,

               I’m Ahmet Kaan. I’m Turkish and was born in 2001. I graduated with a degree in Computer Engineering from Erciyes University. I’m keen on understanding computers as deeply as possible and I love the open‑source philosophy and the KISS principle.

My passion is to create software that can stand the test of time. Developers should write versatile, bug‑free programs. It’s impossible to ignore the impact of software. Today’s world is led by it and will continue to be. As developers, we must consider as many possibilities as we can in order to build robust and secure applications.

I hope to create free and open‑source software that is widely used, allowing people to benefit from it without having to worry about privacy or the reliability of the program.               "
            }
        }
        Footer {}
    }
}
