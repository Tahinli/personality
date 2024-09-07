use dioxus::prelude::*;

use crate::{Footer, Header, Route};

#[component]
pub fn Projects() -> Element {
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
pub fn Project(id: i32) -> Element {
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
                        Strategy is simple. Since normal users don't have a chance to share their ip publicly, 
                        relay server collects data from streamer app. Frontend part's able to connect with relay server and 
                        this allows people listen everywhere where the internet reaches. 
                    "
                    }
                    h3 {"Why I did this ?"}
                    div {
                        "I wanted to learn how to stream sound data."
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
