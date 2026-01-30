use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/casabaldini/")]
    Casabaldini{},
    #[route("/blog/:id")]
    Blog { id: i32 },

    
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
//const HEADER_SVG: Asset = asset!("/assets/header.svg");
const HEADER_SVG: Asset = asset!("/assets/img/index/cafaggiolo.jpg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const LAGO_IMG: Asset = asset!("/assets/img/index/lago.jpg");

pub struct Slider {
    pub id: i32,
    pub titolo: String,
    pub immagine_url: String,
}
fn main() {
    #[cfg(feature = "server")]
    {
        // Usiamo solo ciò che serve davvero
        use dioxus::prelude::*;

        LaunchBuilder::new()
            // Rimuoviamo il .with_cfg problematico. 
            // Dioxus userà le impostazioni di default (porta 8080)
            .launch(App);
    }

    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.7/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}
        Echo {}
    }
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

            // Navigation links
            Link {
                to: Route::Blog { id: id - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                to: Route::Blog { id: id + 1 },
                "Next"
            }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Blog { id: 1 },
                "Blog"
            }
            Link {
                to: Route::Casabaldini {  },
                "Casabaldini"
            }
        }

        Outlet::<Route> {}
    }
}

/// Echo component that demonstrates fullstack server functions.
#[component]
fn Echo() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        div {
            id: "echo",
            h4 { "ServerFn Echo" }
            input {
                placeholder: "Type here to echo...",
                oninput:  move |event| async move {
                    let data = echo_server(event.value()).await.unwrap();
                    response.set(data);
                },
            }

            if !response().is_empty() {
                p {
                    "Server echoed: "
                    i { "{response}" }
                }
            }
        }
    }
}
// casabaldini
#[component]
fn Casabaldini() -> Element {
   rsx! {
        div { style: "font-family: sans-serif; padding: 20px;",
            h1 { "Galleria Dinamica Casabaldini" }
            
            p { 
                if cfg!(target_arch = "wasm32") { 
                    span { style: "color: green;", "✅ CLIENT ATTIVO" }
                } else { 
                    span { style: "color: orange;", "🏠 SERVER RENDERING" }
                }
            }

            div {
                p { "Test immagine con macro asset!:" }
                // 2. USIAMO LA COSTANTE ASSET
                img { src: LAGO_IMG, width: "300" }
            }

            hr {}
            ElencoSliders {}
        }
    }
}

pub async fn get_sliders_test() -> Result<Vec<Slider>, ServerFnError> {
    Ok(vec![
        Slider {
            id: 1,
            titolo: "Vista dal Server".to_string(),
            // Per il server usiamo il percorso che Dioxus si aspetta dopo la build
            immagine_url: LAGO_IMG.to_string(), 
        },
         Slider {
            id: 1,
            titolo: "Vista dal Server".to_string(),
            // Per il server usiamo il percorso che Dioxus si aspetta dopo la build
            immagine_url: "/libera/test.jpg".to_string(), 
        },
        Slider {
            id: 1,
            titolo: "Vista dal Server".to_string(),
            // Per il server usiamo il percorso che Dioxus si aspetta dopo la build
            immagine_url: asset!("/assets/img/index/lagobilancino.jpg").to_string(), 

        },
        Slider {
            id: 1,
            titolo: "Vista dal Server".to_string(),
            // Per il server usiamo il percorso che Dioxus si aspetta dopo la build
            immagine_url: asset!("/assets/img/index/lagobilancinovela.jpg").to_string(), 
        },
        Slider {
            id: 1,
            titolo: "Vista dal Server".to_string(),
            // Per il server usiamo il percorso che Dioxus si aspetta dopo la build
            immagine_url: asset!("/assets/img/index/lagobilancinovela.jpg").to_string(), 
        },
        Slider {
            id: 1,
            titolo: "Vista dal Server".to_string(),
            // Per il server usiamo il percorso che Dioxus si aspetta dopo la build
            immagine_url: asset!("/assets/img/index/loggemedicee.jpg").to_string(), 
        }
    ])
}

pub async fn get_sliders_test_2() -> Result<Vec<Slider>, ServerFnError> {
    Ok(vec![
        Slider {
            id: 1,
            titolo: "Immagine Dinamica (Senza Hash)".to_string(),
            // Usiamo il prefisso '/foto' che abbiamo creato nel server
            immagine_url: "/foto/lago.jpg".to_string(), 
        },
    ])
}
/// Echo the user input on the server.
#[post("/api/echo")]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

#[component]
fn ElencoSliders() -> Element {
    let mut sliders_res = use_resource(move || get_sliders_test());
    let mut count = use_signal(|| 0);
    rsx! {
        button { onclick: move |_| count += 1, "Click test: {count}" }
        match &*sliders_res.read_unchecked() {
            Some(Ok(list)) => rsx! {
                div { style: "display: flex;",
                    for s in list {
                        div { key: "{s.id}", style: "margin: 10px;",
                            h3 { "{s.titolo}" }
                            // Qui usiamo la stringa che arriva dal server
                            img { src: "{s.immagine_url}", width: "200" }
                        }
                    }
                }
            },
            _ => rsx! { p { "Caricamento dati server..." } }
        }
    }
}