#![allow(non_snake_case)]
use dioxus::prelude::*;
// Aggiungiamo esplicitamente FromRow per SQLx
#[cfg(feature = "server")]
use sqlx::FromRow;

// --- IL COMPONENTE APP ---
#[component]
fn App() -> Element {
    let sliders = use_resource(move || get_sliders());

    rsx! {
        h1 { "Benvenuto in DIOXUS CASABALDINI" }
        div { class: "slider-container",
            // 1. Leggiamo la risorsa (sliders.read() restituisce Option<&Result<Vec, E>>)
            match &*sliders.read() {
    Some(Ok(lista_da_db)) => rsx! {
        // Usiamo lista_da_db, NON sliders!
        for s in lista_da_db { 
            div { class: "slide", key: "{s.id}",
                h3 { "{s.titolo}" }
                img { 
                    src: "/public/img/index/{s.img}", 
                    width: "400px",
                }
                p { "{s.testo}" }
            }
        }
    },
    Some(Err(e)) => rsx! { p { "Errore: {e}" } },
    None => rsx! { p { "Caricamento..." } }
}
        }
    }
}

// --- LOGICA DATABASE ---
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, FromRow)] // Aggiunto FromRow qui!
pub struct Slider {
    pub id: i64, 
    pub titolo: String,
    pub testo: String,
    pub img: String,
}

#[server]
pub async fn get_sliders() -> Result<Vec<Slider>, ServerFnError> {
    // 1. Prova il database
    let pool_res = sqlx::SqlitePool::connect("sqlite:casabaldini.sqlite").await;
    
    match pool_res {
        Ok(pool) => {
            let rows = sqlx::query_as::<_, Slider>("SELECT id, titolo, testo, img FROM sliders")
                .fetch_all(&pool)
                .await;
            
            match rows {
                Ok(data) => Ok(data),
                Err(e) => {
                    // Se la query fallisce, restituiamo un dato finto per capire se il server risponde!
                    Ok(vec![Slider { id: 1, titolo: "ERRORE QUERY".into(), testo: e.to_string(), img: "".into() }])
                }
            }
        },
        Err(e) => {
            // Se il DB non si connette, restituiamo questo
            Ok(vec![Slider { id: 0, titolo: "ERRORE DB".into(), testo: e.to_string(), img: "".into() }])
        }
    }
}

// --- MAIN PER IL CLIENT (WASM) ---
#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus::launch(App);
}

// --- MAIN PER IL SERVER (Axum) ---
#[cfg(all(not(target_arch = "wasm32"), feature = "server"))]
#[cfg(all(not(target_arch = "wasm32"), feature = "server"))]
fn main() {
    // 1. Niente router Axum manuale per ora
    // 2. Usiamo solo LaunchBuilder standard
    LaunchBuilder::new().launch(App);
}