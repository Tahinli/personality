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
                "Engineer"}

            div {
                id: "main_about",
                    "
                Hi, I'm Ahmet Kaan. I'm Turk. I was born in 2001.
                I graduated from the computer engineering major at Erciyes University.

                I'm keen on understanding computers as deeply as possible.
                I love open source philosophy and KISS principle.

                My passion is to create software that's able to live as long as possible. Developers should write versatile, bug free programs. Using software is impossible to ignore. World's leading with software and will be. We as developers must think as many possibility as possible to create robust and secure programs.

                I wish I will be able to create free and open source software that is widely used by people without even let them thinking about their privacy and robustness of my program.
                "
            }
        }
        Footer {}
    }
}
