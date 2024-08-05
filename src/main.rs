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
                    div {
                    "In this project any streamer can do their radio stream.
                    Streamer is able to stream audio input and also sound files. 
                    Multiple clients are able to listen streamer. 
                    Relay server connects streamer to worldwide listeners."
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
        footer{
            h4 {
            "Developed by Tahinli with no Frontend Skills"
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
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec consectetur lorem in augue gravida, eget tempus odio consectetur. Vestibulum ut pretium nibh. Praesent id eros id nunc dapibus rutrum. Suspendisse bibendum lacus in massa lobortis, a egestas lectus ullamcorper. Phasellus dignissim augue ac ultricies gravida. Pellentesque bibendum dapibus augue dictum porta. Donec laoreet fermentum dui, non suscipit ante. Nulla ac risus semper, mollis metus vitae, viverra justo. Phasellus ornare diam mi, sed pellentesque est porttitor eget. Proin quis semper mauris, ut maximus est. Vivamus id libero et sapien ullamcorper condimentum. Duis semper elit nibh, ut mattis est auctor nec. Proin porttitor elit arcu, at lobortis nulla lobortis nec. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Sed ligula est, placerat sed felis sit amet, placerat dapibus justo. Duis ac congue purus.

Maecenas interdum velit a neque rutrum vehicula. Phasellus id diam sit amet mauris laoreet fringilla. Nam bibendum felis ac sapien tincidunt fringilla. Nulla luctus orci ut cursus mollis. Nulla malesuada lacus est, at auctor purus semper et. Praesent ipsum neque, tristique ut urna non, volutpat congue turpis. Ut rhoncus mollis turpis in vehicula. Phasellus porta arcu pretium, iaculis lectus eu, pharetra elit. Nullam non tortor justo. In dignissim velit augue, at porta sapien lobortis at. Praesent bibendum facilisis magna et maximus. Integer erat lectus, consequat nec orci a, porta rhoncus magna. Mauris blandit blandit sapien, at gravida lorem finibus in. Vestibulum id purus urna. Nam commodo, augue ut aliquam egestas, magna justo blandit leo, et pulvinar metus velit vel lectus. Curabitur feugiat vulputate sem, nec aliquam dui vestibulum ac.

Integer ut gravida augue. Pellentesque rhoncus magna nibh, at bibendum dui consequat vel. Duis ullamcorper risus eget leo tempus laoreet id at augue. Curabitur cursus vulputate sapien, nec malesuada libero finibus non. Mauris non justo vel arcu scelerisque laoreet in id tellus. Aenean tincidunt semper est, et volutpat nunc ultricies ut. Cras elementum elit nec ante tempus hendrerit. Vestibulum ullamcorper dictum quam nec ullamcorper. In non dolor quis orci vulputate ultricies. Suspendisse viverra risus non maximus vulputate.

Donec sit amet sem ac justo ultrices commodo. Vivamus vel tellus aliquam, interdum libero et, mollis nunc. Maecenas tempor velit ut arcu aliquet vestibulum. Nam quis tincidunt neque. Aenean quis accumsan augue. Ut eleifend luctus nunc, maximus efficitur lorem lacinia non. Nullam imperdiet lectus vel nisl sollicitudin rutrum. Donec luctus sapien ac risus vulputate bibendum. Etiam fermentum leo sit amet posuere ultrices. Morbi quam quam, venenatis in ligula nec, viverra commodo massa. Duis tellus massa, dapibus nec ornare et, porta at lacus. Praesent bibendum iaculis arcu. Donec sed enim tortor. Quisque pharetra tempus condimentum. Phasellus a neque nulla. Nam tincidunt odio odio, tincidunt varius ligula lacinia vel.

Sed ut risus ac erat finibus pretium. Nunc tincidunt porttitor leo ac porttitor. Vivamus ut ultricies purus, elementum feugiat tellus. Mauris at sapien semper, vestibulum risus eget, blandit dolor. Nulla facilisi. Proin faucibus interdum eleifend. Curabitur dictum diam at purus luctus facilisis. Aenean et ligula sapien. Suspendisse non nisl purus. "
            }
        }
        Footer {}
    }
}
