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
                        style: "width: 70%; height: auto;  margin-left: 16%; ",
                    }
                }
            
            }
            img {
                src: HEADER_SVG,
                id: "header",
                style: "width: 85%; height: auto;",
            }

            span { style: "font-size: 1.5em; color: #fffefe;", "Barberino di Mugello" }
            span { style: "font-size: 1em; color: #fffefe;", "2,5 Km. dall'uscita dell'Autostrada A1 " }
            span { style: "font-size: 1.5em; color: #fffefe;", "a pochi Km. da Firenze " }
            span { style: "font-size: 1.5em; color: #fffefe;",

                b { "______________________________________________________" }
            }
            span { style: "font-size: 1em; color: #fffefe;",

                b { "  Per informazioni e prenotazioni telefona al +39 3207060411" }
            }
        }
    }
}