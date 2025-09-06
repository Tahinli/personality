use dioxus::prelude::*;
use personality::Route;
use tracing::Level;

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
