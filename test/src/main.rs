use dioxus::{fullstack::reqwest::Url, prelude::*};
use serde::{Serialize, Deserialize};
use dioxus::prelude::asset;
//use web_sys::Url;
use crate::document::eval;
//use web_sys::console::count;
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

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/css/main.css");
const SLIDERMIN_CSS: Asset = asset!("/assets/home/dist/css/slider-pro.min.css");
const SLIDER_CSS: Asset = asset!("/assets/home/dist/css/slider-pro.css");
const EXAMPLE_CSS: Asset = asset!("/assets/home/dist/css/examples.css");
const MENU_CSS: Asset = asset!("/assets/menu_6/css/default.css");
const HEADER_SVG: Asset = asset!("/assets/img/index/cafaggiolo.jpg");
const TAILWIND_CSS: Asset = asset!("/assets/css/tailwind.css");
//const ACE_MENU_CSS: Asset = asset!("/assets/Ace-Menu/css/demo.css");
//const ACE_MENU_RESP: Asset = asset!("/assets/Ace-Menu/css/ace-responsive-menu.css");
//const LAGO_IMG: Asset = asset!("/assets/img/index/lago.jpg");
const JQUERY_JS: Asset = asset!("/assets/home/dist/js/jquery.sliderPro.min.js");
//const ACE_RESP_JS: Asset = asset!("/assets/Ace-Menu/js/ace-responsive-menu.js");
//const ACE_JS: Asset = asset!("/assets/Ace-Menu/js/jquery-1.10.1.min.js");
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
        document::Link { rel: "stylesheet", href: EXAMPLE_CSS }
        document::Link { rel: "stylesheet", href: MENU_CSS }
        document::Link { rel: "stylesheet", href: SLIDERMIN_CSS }
        document::Link { rel: "stylesheet", href: SLIDER_CSS }
        //document::Script { src: ACE_RESP_JS }
        //document::Script { src: ACE_JS }
        document::Script { src: JQUERY_JS }
        
        document::Link { rel: "icon", href: FAVICON }
        
        //document::Link { rel: "stylesheet", href: ACE_MENU_CSS }
        //document::Link { rel: "stylesheet", href: ACE_MENU_RESP }
        document:: Meta {name:"viewport", content:"width:device-width, user-scalable:no,initial-scale:1.0, minimum-scale:1.0, maximum-scale:1.0"}
        document::Link { rel: "stylesheet", href: MAIN_CSS } 
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
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

