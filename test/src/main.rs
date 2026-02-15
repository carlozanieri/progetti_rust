use dioxus::{fullstack::reqwest::Url, prelude::*};
use serde::{Serialize, Deserialize};
use dioxus::prelude::asset;
//use web_sys::Url;
use crate::document::eval;
mod config;
mod models;
mod components; // Questo caricherà components/mod.rs
use crate::models::get_menu_db;
use crate::models::get_submenu_db;

use components::casabaldini::Casabaldini;
use components::navbar::Navbar;
use components::blog::Blog;
use components::echo::Echo;
use components::hero::Hero;
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

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)] 
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct Menus {
	pub id:       i64,
	pub codice:   String,
	pub radice:   String,
	pub livello:  i64,
	pub titolo:   String,
	pub link:     String,
    pub ordine:   i64,
	
}

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)] 
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct Submenus{
	pub id:       i64,
	pub codice:   String,
	pub radice:   String,
	pub livello:  i64,
	pub titolo:   String,
	pub link:     String,
    pub ordine:   i64,
	
}

#[derive(Props, Clone, PartialEq)]
pub struct NavItemProps {
    m: Menus,
    subitems: Vec<Submenus>,
}

fn main() {
        
    dioxus::launch(App);
}

#[component]
fn App() -> Element {

    // 1. Definiamo l'effetto PRIMA del rsx!
    rsx! {
        document::Script { src: "https://code.jquery.com/jquery-3.6.2.min.js" }
        document::Link { rel: "stylesheet", href: crate::config::EXAMPLE_CSS }
        document::Link { rel: "stylesheet", href: crate::config::MENU_CSS }
        document::Link { rel: "stylesheet", href: crate::config::SLIDERMIN_CSS }
        document::Link { rel: "stylesheet", href: crate::config::SLIDER_CSS }
        //document::Script { src: ACE_RESP_JS }
        //document::Script { src: ACE_JS }
        document::Script { src: crate::config::JQUERY_JS }
        
        document::Link { rel: "icon", href: crate::config::FAVICON }
        
        //document::Link { rel: "stylesheet", href: ACE_MENU_CSS }
        //document::Link { rel: "stylesheet", href: ACE_MENU_RESP }
        document:: Meta {name:"viewport", content:"width:device-width, user-scalable:no,initial-scale:1.0, minimum-scale:1.0, maximum-scale:1.0"}
        document::Link { rel: "stylesheet", href: crate::config::MAIN_CSS } 
        document::Link { rel: "stylesheet", href: crate::config::TAILWIND_CSS }
        //Navbar {}  
        Router::<Route> {}
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

