use dioxus::prelude::*;

use crate::{Footer, Header, Route};

#[component]
pub fn Projects() -> Element {
    rsx! {
        Header {}
        div {
            class: "projects_list",
            h1 {"Projects"}
            div { class: "project_link", Link {to: Route::Project { id: 1 }, class: "project_link", "Radioxide"} }
            div { class: "project_link", Link {to: Route::Project { id: 2 }, class: "project_link", "Memory Filler Killer"} }
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
                        Strategy is simple. Since normal users don't have a chance to share their IP publicly, 
                        relay server collects data from streamer app. Frontend part's able to connect with relay server and 
                        this allows people listen everywhere where the internet reaches. 
                    "
                    }
                    h3 {"Why I did this ?"}
                    div {
                        "I wanted to learn how to stream sound data."
                    }

                    h3 {"What I learned ?"}
                    div {
                        "Websocket, TLS, Stream Optimization, Sound Mix, How Sound Works, ELM Architecture"
                    }
                }
            }
        }

        2 => {
            rsx! {
                div {
                    class:"project",
                    h1 {"Memory Filler Killer"}
                    h5 {"Memory fullness checker written in rust "}
                    Link {id: "github_link_in_project_page", to: "https://github.com/Tahinli/rust-memory-filler-killer", "GitHub"}
                    div {
                    "
                        In this project a program checks for available memory.
                        If available memory gets lower then given value, program tries to find memory filler.
                        When memory filler found by the program, it will be killed to keep operating system alive.
                    "
                    }
                    h3 {"Why I did this ?"}
                    div {
                        "Linux normally has this kind of feature. 
                        If it lacks of memory to keep OS alive then program that uses most of the memory be killed by kernel.
                        Problem is it's not working well. I don't know why but 
                        I got in a situation like a program consumes all of my memory by some kind of leak but kernel couldn't do anything and my system crashed.
                        I wanted to solve this problem, now I use my program as a service for my own system to prevent myself from being in same situation."
                    }

                    h3 {"What I learned ?"}
                    div {
                        "Interacting with OS and processes via sysinfo crate."
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
