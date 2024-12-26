use dioxus::prelude::*;
use home::Home;
use posts::{Post, Posts};
use projects::{Project, Projects};

pub mod home;
pub mod posts;
pub mod projects;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/projects")]
    Projects {},
    #[route("/projects/:id")]
    Project { id: i32 },
    #[route("/posts")]
    Posts {},
    #[route("/posts/:id")]
    Post { id: i32 },
}

#[component]
pub fn Header() -> Element {
    rsx! {
        header {
                nav {
                    class: "main_nav",
                    Link {to: Route::Home{}, class:"main_nav_btn", "Home"}
                    Link {to: Route::Projects{}, class:"main_nav_btn", "Projects"}
                    Link {to: Route::Posts {}, class:"main_nav_btn", "Posts"}
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
