use crate::{Footer, Header, Route};
use dioxus::prelude::*;

#[component]
pub fn Writings() -> Element {
    rsx! {
        Header {}
        div {
            class: "posts_list",
            h1 {"Writings"}
            div { class: "post_link", Link {to: Route::Writing { id: 1 },class: "post_link", "Awaken"} }
        }
        Footer {}
    }
}

#[component]
pub fn Writing(id: i32) -> Element {
    let x = match id {
        1 => {
            rsx! {
                div {
                    class:"post",
                    h1 {"Awaken"}
                    br {  }
                    div {
"Does every word flies, if it's not written."
br {  }
"A life goes by a word that is written."
br {  }
"Word doesn't know what is written."
br {  }
"Only feels why it's written."
br {  }
br {  }
"Things come and go with a light."
br {  }
"A cry is woken by the light."
br {  }
"Does ever the light's gone for a blind."
br {  }
"Maybe awaken is blinded by the light."
br {  }
br {  }

"Time goes with a paper and blood."
br {  }
"The blood will end by the paper and a heart."
br {  }
"The paper came with the blood and a knife."
br {  }
"Hearts will die with the time and the blood."
br {  }
br {  }

"Sirens are here to wake."
br {  }
"The awaken is ready to take a break."
br {  }
"Screams goes where, when a silence comes to take."
br {  }
"Take the gap to get a moment to wake."
br {  }
br {  }

"The awaken can't feel a thing what is real."
br {  }
"Does ever be a thing that is not surreal."
br {  }
"Real isn't the moment to be there to deal."
br {  }
"Deal is ever be a thing in real."
br {  }
br {  }

"Blooded writes these down with a blind mind."
br {  }
"A mind came by the light to get a heart of a knife."
br {  }
"The knife cuts the light in a written time."
br {  }
"The written is the siren for the awaken to die."
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
