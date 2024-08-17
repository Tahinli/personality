use dioxus::prelude::*;
use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/projects/")]
    Projects {},
    #[route("/projects/:id")]
    Project { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(app);
}

fn app() -> Element {
    rsx! {
        link {
            rel: "stylesheet",
            style { {include_str!("../assets/main.css")} },
        }
        Router::<Route> {}
    }
}

#[component]
fn Projects() -> Element {
    rsx! {
        Header {}
        div {
            class: "projects_list",
            h1 {"Projects"}
            Link {to: Route::Project { id: 1 }, class: "project_link", "Radioxide"}
        }
        Footer {}
    }
}
#[component]
fn Project(id: i32) -> Element {
    let x = match id {
        1 => {
            rsx! {
                div {
                    class:"project",
                    h1 {"Radioxide"}
                    h5 {"Online radio written in Rust."}
                    Link {id: "github_link_in_project_page", to: "https://github.com/tahinli/radioxide", "GitHub"}
                    div {
                    "
                        In this project any streamer can do their radio stream.
                        Streamer is able to stream audio input and also sound files. 
                        Multiple clients are able to listen streamer. 
                        Relay server connects streamer to worldwide listeners.
                    "
                    }
                }
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


#[component]
fn Header() -> Element {
    rsx!{
        header {
                nav {
                    class: "main_nav",
                    Link {to: Route::Home{}, class:"main_nav_btn", "Home"}
                    Link {to: Route::Projects{}, class:"main_nav_btn", "Projects"}
                }
        }
        div { id: "content",
            Outlet::<Route> {}
        }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        div {class:"footer_div",
            footer{
                h4 {
                "Developed by Tahinli with Rust + WASM without Frontend Skills"
                }
            }
        }
        
    }
}

#[component]
fn Home() -> Element {
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
                "System && Backend Developer"}
                
            div {
                id: "social_links",
                Link {id:"github",to: "https://github.com/tahinli", "GitHub"}
            
                Link {id:"linkedin",to: "https://linkedin.com/in/ahmetkaangumus", "LinkedIn"}
            }
                
            div {
                id: "main_about",
                    "
                Hi, I'm Ahmet Kaan. I'm Turk. I was born in 2001.
                I'm currently studying on Computer Science major in Erciyes University.

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
