use crate::prelude::*;
use crate::components::linkutili::Linkutili;

#[component]

pub fn Hero() -> Element {
    const HEADER_SVG: Asset = asset!("/assets/img/index/fronte.jpg");
    const logo_SVG: Asset = asset!("/assets/logo.jpg");
    rsx! {
        div { id: "hero",
            div { id: "links", style: " align-items:center;",
                a { href: "/casabaldini/index",
                    img {
                        src: logo_SVG,
                        style: "width: 65%; height: auto;  margin-left: 20%; ",
                    }
                }
            
            }
            img {
                src: HEADER_SVG,
                id: "header",
                style: "width: 85%; height: auto;",
            }
        
        }
    }
}