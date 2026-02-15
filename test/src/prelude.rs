pub use dioxus::{fullstack::reqwest::Url, prelude::*};
pub use serde::{Serialize, Deserialize};
pub use dioxus::prelude::asset;
pub use dioxus::prelude::*;
pub use crate::models::{Menus, Submenus};
pub use crate::components::nav_item::NavItem;
pub use crate::components::elencosliders::ElencoSliders;
pub use crate::models::get_menu_db;
pub use crate::models::get_submenu_db;
pub use crate::Route;
pub use crate::models::get_sliders_db;
pub use crate::document::eval;
pub mod config;
pub mod models;
pub mod components; // Questo caricherà components/mod.rs
pub use crate::models::get_menu_db;
pub use crate::models::get_submenu_db;
//use crate::Route;
pub use components::casabaldini::Casabaldini;
pub use components::navbar::Navbar;
pub use components::blog::Blog;
pub use components::echo::Echo;
pub use components::hero::Hero;
pub use crate::models::{Menus, Submenus};
pub use crate::components::nav_item::NavItem;
pub use crate::components::elencosliders::ElencoSliders;
pub use crate::models::get_menu_db::*;
pub use crate::models::get_submenu_db::*;
pub use crate::Route;
pub use crate::models::get_sliders_db::*;
#[cfg(not(target_arch = "wasm32"))]
pub use sqlx::{PgPool, FromRow}; // Cambiato da SqlitePool a PgPool