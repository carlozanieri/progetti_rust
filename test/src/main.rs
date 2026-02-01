use dioxus::prelude::*;
use serde::{Serialize, Deserialize};
//use dioxus::fullstack::prelude::*;
use dioxus::prelude::asset;
#[cfg(not(target_arch = "wasm32"))]
use sqlx::{SqlitePool, FromRow};
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] 
// Questa riga dice: aggiungi FromRow solo se NON siamo su WASM
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct Slider {
    pub id: i32,
    pub img: String,
    pub titolo: String,
   
}
fn main() {
    // Questo è il modo più pulito in 0.7 per far funzionare tutto
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
    let sliders = use_resource(move || get_sliders_db());
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

/// Echo the user input on the server.
#[post("/api/echo")]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}


// 2. LA FUNZIONE SERVER (Il ponte verso il disco)

#[server]
pub async fn get_sliders_test() -> Result<Vec<Slider>, ServerFnError> {
    use std::fs;
    use base64::{Engine as _, engine::general_purpose};

    // Qui il server legge il file dal TUO disco
    let path = "assets/img/index/lagobilancino.jpg";
    
    let img = match fs::read(path) {
        Ok(bytes) => {
            let b64 = general_purpose::STANDARD.encode(bytes);
            format!("data:image/jpeg;base64,{}", b64)
        },
        Err(_) => "https://via.placeholder.com/400?text=Immagine+non+trovata".to_string(),
    };

    Ok(vec![
        Slider {
            id: 1,
            titolo: "Test Dinamico".to_string(),
            img,
        },
    ])
}

#[server]
pub async fn get_sliders_db() -> Result<Vec<Slider>, ServerFnError> {
    // Import necessari solo lato server
    #[cfg(not(target_arch = "wasm32"))]
    {
        use sqlx::SqlitePool;
        use std::fs;
        use base64::{Engine as _, engine::general_purpose};

        // 1. Connessione al pool (sqlite:nome_file)
        let pool = SqlitePool::connect("sqlite:casabaldini.sqlite").await
            .map_err(|e| ServerFnError::new(format!("Errore connessione DB: {}", e)))?;

        // 2. Query al database mappata sulla struct Slider
        let mut rows: Vec<Slider> = sqlx::query_as::<_, Slider>("SELECT id, titolo, img FROM sliders")
            .fetch_all(&pool)
            .await
            .map_err(|e| ServerFnError::new(format!("Errore query: {}", e)))?;

        // 3. Trasformazione delle immagini in Base64
        // Cicliamo ogni riga e sostituiamo il nome file con i dati reali
        for slider in &mut rows {
            let path = format!("assets/img/index/{}", slider.img);
            if let Ok(bytes) = fs::read(&path) {
                let b64 = general_purpose::STANDARD.encode(bytes);
                slider.img = format!("data:image/jpeg;base64,{}", b64);
            } else {
                // Se il file non esiste, mettiamo un placeholder per non rompere il sito
                slider.img = "https://via.placeholder.com/400?text=Immagine+Non+Trovata".to_string();
            }
        }

        Ok(rows)
    }

    // Risposta "fantoccio" per il compilatore WASM (lato client)
    #[cfg(target_arch = "wasm32")]
    Ok(vec![])
}
#[component]
fn ElencoSliders() -> Element {
    let mut sliders_res = use_resource(move || get_sliders_db());
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
                            img { src: "{s.img}", width: "200" }
                        }
                    }
                }
            },
            _ => rsx! { p { "Caricamento dati server..." } }
        }
    }
}