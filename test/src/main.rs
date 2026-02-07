use dioxus::{fullstack::reqwest::Url, prelude::*};
use serde::{Serialize, Deserialize};
use dioxus::prelude::asset;
//use web_sys::Url;
use crate::document::eval;
//use web_sys::console::count;

#[cfg(not(target_arch = "wasm32"))]
use sqlx::{PgPool, FromRow}; // Cambiato da SqlitePool a PgPool
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
const SLIDERMIN_CSS: Asset = asset!("/assets/home/dist/css/slider-pro.min.css");
const SLIDER_CSS: Asset = asset!("/assets/home/dist/css/slider-pro.css");
const EXAMPLE_CSS: Asset = asset!("/assets/home/dist/css/examples.css");
const MENU_CSS: Asset = asset!("/assets/menu_6/css/default.css");
const HEADER_SVG: Asset = asset!("/assets/img/index/cafaggiolo.jpg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const LAGO_IMG: Asset = asset!("/assets/img/index/lago.jpg");
const JQUERY_JS: Asset = asset!("/assets/home/dist/js/jquery.sliderPro.min.js");
//const MIO_JS: Asset = asset!("/assets/home/dist/js/mio.js");
const DB_URL: &str = "postgres://carlo:treX39@57.131.31.228:5432/casabaldini";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] 
// Questa riga dice: aggiungi FromRow solo se NON siamo su WASM
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct Slider {
    pub id: i64,
    pub img: String,
    pub titolo: String,
    pub testo: String,
    pub caption: String,
}
fn main() {
    // Questo è il modo più pulito in 0.7 per far funzionare tutto
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // 1. Definiamo l'effetto PRIMA del rsx!
    rsx! {
        document::Script { src: "https://code.jquery.com/jquery-3.6.2.min.js" }
        document::Link { rel: "stylesheet", href: EXAMPLE_CSS }
        document::Link { rel: "stylesheet", href: MENU_CSS }
        document::Link { rel: "stylesheet", href: SLIDERMIN_CSS }
        document::Link { rel: "stylesheet", href: SLIDER_CSS }
        document::Script { src: JQUERY_JS }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } 
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
          
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
    //let document = window().unwrap().document().unwrap();
    let sliders = use_resource(move || get_sliders_db());
    
   rsx! {
           
     
    
            div { class:"slider-pro", 
            h1 { "Galleria Dinamica Casabaldini" }
            
            p { 
                if cfg!(target_arch = "wasm32") { 
                    span { style: "color: green;", "✅ CLIENT ATTIVO" }
                } else { 
                    span { style: "color: orange;", "🏠 SERVER RENDERING" }
                }
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
pub async fn get_sliders_db() -> Result<Vec<Slider>, ServerFnError> {
    // Trasformiamo l'errore di connessione e di query in stringhe leggibili da ServerFnError
    let pool = PgPool::connect(DB_URL)
        .await
        .map_err(|e| ServerFnError::new(format!("Errore connessione DB: {}", e)))?;

    let rows = sqlx::query_as::<_, Slider>("SELECT id, titolo, img, testo, caption FROM sliders")
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Errore query: {}", e)))?;
    println!("📡 Server: Row recuperate, invio in corso...");
    Ok(rows)
}

#[server]
pub async fn get_single_image_b64(name: String) -> Result<String, ServerFnError> {
    use base64::{Engine as _, engine::general_purpose};
    let path = format!("assets/img/index/{}", name);
    let bytes = std::fs::read(path).map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(format!("data:image/jpeg;base64,{}", general_purpose::STANDARD.encode(bytes)))
}

#[component]
fn ElencoSliders() -> Element {
    let sliders_res = use_resource(move || get_sliders_db());

    // Spostiamo la logica di inizializzazione in una funzione richiamabile
    let inizializza_slider = move |_| {
        spawn(async move {
            let _ = eval(r#"
                console.log("🔍 Controllo DOM in corso...");
                var $slider = $('#example1');
                
                console.log("Elementi trovati:", $slider.length);
                console.log("HTML interno:", $slider.html() ? $slider.html().substring(0, 100) : "VUOTO");
                console.log("jQuery presente:", typeof jQuery !== 'undefined');
                console.log("Plugin SliderPro presente:", typeof $.fn.sliderPro !== 'undefined');

                if ($slider.length > 0 && typeof $.fn.sliderPro !== 'undefined') {
                    console.log("🚀 TENTATIVO INIZIALIZZAZIONE...");
                    $slider.sliderPro({
                        width: 960,
                        height: 500,
                        arrows: true,
                        autoplay: true,
                        autoHeight: false,
                        forceSize: 'fullWidth'
                    });
                    console.log("✅ Chiamata sliderPro effettuata");
                } else {
                    console.error("❌ Errore critico: Slider o Plugin non trovati");
                }
            "#);
        });
    };

    rsx! {
        match &*sliders_res.read_unchecked() {
            Some(Ok(list)) => rsx! {
                div { 
                    id: "example1", 
                    class: "slider-pro",
                    // IMPORTANTE: Quando il div appare nel DOM con i dati, 
                    // chiamiamo l'inizializzazione
                    onmounted: inizializza_slider, 
                    
                    div { class: "sp-slides",
                        for s in list {
                            div { class: "sp-slide", key: "{s.id}",
                                h3 { class: "sp-layer sp-black sp-padding", "{s.titolo}" }
                                FastImage { name: s.img.clone() }
                                p { class: "sp-layer sp-white sp-padding", "{s.testo}" }
                            }
                        }
                    }
                }
            },
            _ => rsx! { p { "Caricamento in corso..." } }
        }
    }
}

#[component]
fn FastImage(name: String) -> Element {
    let mut img_data = use_signal(|| String::new());

    // Cloniamo 'name' qui per la risorsa, lasciando l'originale intatto per il rsx!
    let name_for_resource = name.clone();

    use_resource(move || {
        let n = name_for_resource.clone();
        async move {
            if let Ok(b64) = get_single_image_b64(n).await {
                img_data.set(b64);
            }
        }
    });

    if img_data().is_empty() {
        rsx! { 
            div { 
                class: "sp-image", 
                style: "width: 250px; height: 500px; background: #222;", 
                "..." 
            } 
        }
    } else {
        rsx! { 
            img { 
                // Ora 'name' è disponibile perché non è stato "mosso" sopra
                key: "{name}",
                class: "sp-image", 
                src: "{img_data}", 
                width: "250",
                "loading": "eager",
                style: "max-width: 110%; height: 110%;",
            } 
        }
    }
}