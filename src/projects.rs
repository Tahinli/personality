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
            div { class: "project_link", Link {to: Route::Project { id: 2 }, class: "project_link", "TCP File Transfer"} }
            div { class: "project_link", Link {to: Route::Project { id: 3 }, class: "project_link", "Memory Filler Killer"} }
            div { class: "project_link", Link {to: Route::Project { id: 4 }, class: "project_link", "Package Manager"} }
            div { class: "project_link", Link {to: Route::Project { id: 5 }, class: "project_link", "Personality"} }
            div { class: "project_link", Link {to: Route::Project { id: 6 }, class: "project_link", "UDP Hole Puncher"} }
            div { class: "project_link", Link {to: Route::Project { id: 7 }, class: "project_link", "Blockchain"} }
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
                        relay server collects data from streamer app. Front end part's able to connect with relay server and 
                        this allows people listen everywhere where the internet reaches. 
                    "
                    }
                    h3 {"Why I did this ?"}
                    div {
                        "I wanted to learn how to stream sound data."
                    }

                    h3 {"What I learned ?"}
                    div {
                        "WebSocket, TLS, Stream Optimization, Sound Mix, How Sound Works, ELM Architecture"
                    }
                }
            }
        }

        2 => {
            rsx! {
                div {
                    class:"project",
                    h1 {"TCP File Transfer"}
                    h5 {"New file transfer protocol based on TCP, written in Rust"}
                    Link {id: "github_link_in_project_page", to: "https://github.com/Tahinli/rust-tcp-file-transfer", "GitHub"}
                    div {
                    "
                        In this project I wanted to create a program for transferring files with network. 
                        I implemented this to use personally, especially in local networks. 
                        Basic client server structure both client and server can receive or send data.
                    "
                    }
                    h3 {"Why I did this ?"}
                    div {
                        "
                            I wanted a portable easy way to transfer files between different operating systems. 
                            I personally experienced many problem while I was working for Erciyes University
                            because of cross platform support and lack of portability of other programs.
                            That's why I wanted to solve it.
                        "
                    }

                    h3 {"What I learned ?"}
                    div {
                        "
                            Interacting with network in Rust.
                        "
                    }

                    h3 {"Note"}
                    div {
                        "
                            Additionally it's my first project in Rust. That's why after even learning much better ways to implement this program, 
                            I don't change the code. I keep this program as a memento and a milestone for my improvement.
                        "
                    }
                }
            }
        }
        3 => {
            rsx! {
                div {
                    class:"project",
                    h1 {"Memory Filler Killer"}
                    h5 {"Memory fullness and filler checker written in Rust "}
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
                        "
                            Linux normally has this kind of feature.
                            If it lacks of memory to keep OS alive then program that uses most of the memory be killed by kernel.
                            Problem is it's not working well. I don't know why but 
                            I got in a situation like a program consumes all of my memory by some kind of leak but kernel couldn't do anything and my system crashed.
                            I wanted to solve this problem, now I use my program as a service for my own system to prevent myself from being in same situation.
                        "
                    }

                    h3 {"What I learned ?"}
                    div {
                        "Interacting with operating system and processes for reading data and sending signals."
                    }
                }
            }
        }
        4 => {
            rsx! {
                div {
                    class:"project",
                    h1 {"Package Manager"}
                    h5 {"Package manager written in Rust"}
                    Link {id: "github_link_in_project_page", to: "https://github.com/Tahinli/rust-package-manager", "GitHub"}
                    div {
                    "
                        In this project user can upload own packages to server then able to update and delete.
                        Other users can download and install uploaded packages to their systems. 
                        They are able to update and delete packages. 
                        Also client side supports updating all packages that is already installed on the system. 
                    "
                    }
                    h3 {"Why I did this ?"}
                    div {
                        "
                            I wanted to learn how can I build package manager since I use them everyday.
                        "
                    }

                    h3 {"What I learned ?"}
                    div {
                        "Seems like I didn't learn new things but made stronger what I knew."
                    }
                }
            }
        }
        5 => {
            rsx! {
                div {
                    class:"project",
                    h1 {"Personality"}
                    h5 {"Personal website written in Rust"}
                    Link {id: "github_link_in_project_page", to: "https://github.com/Tahinli/personality", "GitHub"}
                    div {
                    "
                        In this project I wanted to build portfolio website for myself and currently you're viewing what is built.
                        There are couple pages and sections. It's a static page based on WASM.
                    "
                    }
                    h3 {"Why I did this ?"}
                    div {
                        "
                            I wanted to learn HTML and CSS a bit also having website as a document of yourself sounds nice.
                        "
                    }

                    h3 {"What I learned ?"}
                    div {
                        "
                            Little bit HTML and CSS. I wish I could create better views but doing this kind of things always hard for me.
                        "
                    }
                }
            }
        }
        6 => {
            rsx! {
                div {
                    class:"project",
                    h1 {"UDP Hole Puncher"}
                    h5 {"UDP hole puncher written in Rust"}
                    Link {id: "github_link_in_project_page", to: "https://github.com/Tahinli/rust-udp-hole-puncher", "GitHub"}
                    div {
                    "
                        In this project I wanted to bypass NAT (Network Address Translation) limitations. Because of lack of IPv4 addresses and security reasons we have public and private addresses.
                        In this situation, we can't easily create peer to peer connection unfortunately. Thanks to UDP hole punching strategy, we may bypass NAT restrictions. 
                        I can't say it works every time because NAT strategies are getting complex by different layers and even creating a hole for your router not enough for peer connection.
                    "
                    }
                    h3 {"Why I did this ?"}
                    div {
                        "
                            I wanted to create peer to peer connection without sending actual data to some relay server. UDP hole punching still rely on at least a connectible address (let's say a relay) but just for creating a hole not for sending all data.
                        "
                    }

                    h3 {"What I learned ?"}
                    div {
                        "
                            I learnt more details and different type of NATs. Working with UDP is also another plus.
                        "
                    }
                }
            }
        }
        7 => {
            rsx! {
                div {
                    class:"project",
                    h1 {"Blockchain"}
                    h5 {"Blockchain written in Rust"}
                    Link {id: "github_link_in_project_page", to: "https://github.com/Tahinli/rust-blockchain", "GitHub"}
                    div {
                    "
                        In this project I wanted to implement a blockchain structure from scratch. Basically project starts with defining block, blockchain, genesis block so on. After that I created network strategy to transfer these blocks to miners. When miners mine blocks, server tries to collect data from them and decides what next blocks will be by consensus mechanism.                       
                    "
                    }
                    h3 {"Why I did this ?"}
                    div {
                        "
                            Basically I wanted to learn how things work in blockchain side.
                        "
                    }

                    h3 {"What I learned ?"}
                    div {
                        "
                            I realised pitfalls of the blockchain technology more clearly by the first hand.
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
