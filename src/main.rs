use dioxus::prelude::*;
use personality::Route;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx! {
        document::Meta { name: "darkreader-lock" }
        link {
            rel: "stylesheet",
            style { {include_str!("../assets/main.css")} },
        }
        Router::<Route> {}
    }
}
