pub use dioxus::{fullstack::reqwest::Url, prelude::*};
pub use serde::{Serialize, Deserialize};
pub use dioxus::prelude::asset;
pub use dioxus::prelude::*;
pub use crate::models::{Menus, Submenus};
pub use crate::components::nav_item::NavItem;
pub use crate::components::elencosliders::ElencoSliders;
pub use crate::document::eval;
 // Questo caricherà components/mod.rs

//use crate::Route;
#[cfg(not(target_arch = "wasm32"))]
pub use sqlx::{PgPool, FromRow}; // Cambiato da SqlitePool a PgPool