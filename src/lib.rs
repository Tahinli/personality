use dioxus::prelude::*;
use home::Home;
use projects::{Project, Projects};
use publications::{Publication, Publications};

pub mod home;
pub mod projects;
pub mod publications;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/projects/")]
    Projects {},
    #[route("/projects/:id")]
    Project { id: i32 },
    #[route("/publications/")]
    Publications {},
    #[route("/publications/:id")]
    Publication { id: i32 },
}

#[component]
pub fn Header() -> Element {
    rsx! {
        header {
                nav {
                    class: "main_nav",
                    Link {to: Route::Home{}, class:"main_nav_btn", "Home"}
                    Link {to: Route::Projects{}, class:"main_nav_btn", "Projects"}
                    Link {to: Route::Publications{}, class:"main_nav_btn", "Publications"}
                }
        }
        div { id: "content",
            Outlet::<Route> {}
        }
    }
}

#[component]
pub fn Footer() -> Element {
    rsx! {
        div {class:"footer_div",
            footer{
                h4 {
                "Developed by Tahinli with Rust + WASM without Front End Skills"
                }
            }
        }

    }
}
