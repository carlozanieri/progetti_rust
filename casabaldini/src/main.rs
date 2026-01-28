#![allow(non_snake_case)]
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// 1. DICHIARIAMO L'ASSET (Come fa Dioxus nei progetti nuovi)
// Questo dice a Dioxus: "Prepara questo file perché mi servirà!"
const LAGO_IMG: Asset = asset!("/assets/img/index/lago.jpg");

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Slider {
    pub id: i32,
    pub titolo: String,
    pub immagine_url: String,
}

fn main() {
    LaunchBuilder::new().launch(App);
}

fn App() -> Element {
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

#[component]
fn ElencoSliders() -> Element {
    let mut sliders_res = use_resource(move || get_sliders_test());

    rsx! {
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

#[server]
pub async fn get_sliders_test() -> Result<Vec<Slider>, ServerFnError> {
    Ok(vec![
        Slider {
            id: 1,
            titolo: "Vista dal Server".to_string(),
            // Per il server usiamo il percorso che Dioxus si aspetta dopo la build
            immagine_url: LAGO_IMG.to_string(), 
        },
    ])
}