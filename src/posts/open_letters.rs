use dioxus::prelude::*;
use crate::{Footer, Header, Route};


#[component]
pub fn OpenLetters() -> Element {
    rsx! {
        Header {}
        div {
            class: "posts_list",
            h1 {"Open Letters"}
            div { class: "post_link", Link {to: Route::OpenLetter { id: 1 },/*   */class: "post_link", "16 February 2025"} }
            div { class: "post_link", Link {to: Route::OpenLetter { id: 2 }, class: "post_link", "24 August 2025"} }
        }
        Footer {}
    }
}

#[component]
pub fn OpenLetter(id: i32) -> Element {
    let x = match id {
        1 => {
            rsx! {
                div {
                    class:"post",
                    h1 {"16 February 2025"}
                    br {  }
                    div {
                        "Hi,"
                        br {  }
                        "Since I'm an open-source developer and a person who relies on open source software, I realize there is a massive contradiction to my perspective and my life. It's GitHub, which is run by Microsoft. It took me a lot of time to understand what I'm doing after using it for many years and pushing thousands of commits for tens of projects."
                        br {  }
                        "I don't want this situation to continue. From now on,"

                        br {  }
                        br {  }
                        p { "I have archived all of my repositories and plan to log in only for opening issues and pull requests for other projects." }

                        br {  }
                        p { "My new projects will not be hosted in there." }

                        br {  }
                        p { "If an existing project needs an update, or I want to add new things, it will not be in there." }

                        br {  }
                        "You can follow my projects and source codes from https://tahinli.com."

                        br {  }
                        br {  }
                        "Best wishes,"

                        br {  }
                        br {  }
                        "Ahmet Kaan Gümüş"
                    }
                }
            }
        },
        2 => {
            rsx! {
                div {
                    class:"post",
                    h1 {"24 August 2025"}
                    br {}
                    div {
                        "Hi,"
                        br {  }
                        "It has been couple of months since I quit from GitHub. In this period I understand couple of things,"
                        
                        br {  }
                        br {  }
                        p { "Only(or mostly) big open-source projects are capable of reaching others and maintaining themselves without using GitHub." }

                        br {  }
                        p{ "If you are a solo developer or a small team, you already lose your chance to reach others when you self-host." }

                        br {  }
                        p { "Data security, backup and recovery processes are more serious things than they seemed."}

                        br {  }
                        p { "CI/CD processes are not free." }

                        br {  }
                        p { "Even non technical people are familiar with GitHub, and it's an indicator for HR departments while applying a job." }

                        br {  }
                        br {  }
                        "I'm still in the same position as in my 16 February 2025 letter. GitHub is still closed source and owned by Microsoft. I want to own my own code but still need to reach community, get better CI/CD and have reliable backup opportunities. So from now on,"
                        br {  }
                        br {  }
                        p { "I will mirror my repos into different platforms, included GitHub." }

                        br {  }
                        "Best wishes,"
                        br {  }
                        br {  }
                        "Ahmet Kaan Gümüş"
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
