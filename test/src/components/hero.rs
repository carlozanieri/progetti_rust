use dioxus::prelude::*;
use crate::models::{Menus, Submenus};
use dioxus::prelude::*;
use crate::components::nav_item::NavItem;
use crate::models::get_menu_db;
use crate::models::get_submenu_db;
use crate::models::get_single_image_b64;
use crate::Route;
use dioxus::{fullstack::reqwest::Url, prelude::*};
use serde::{Serialize, Deserialize};

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