use crate::prelude::*;
pub use crate::models::get_sliders_db;
use crate::components::nav_item::NavItem;
use crate::components::elencosliders::ElencoSliders;
#[component]
pub fn Casabaldini() -> Element {
    //let document = window().unwrap().document().unwrap();
    let sliders = use_resource(move || get_sliders_db());
    rsx! {
        
     
    
            div { class:"slider-pro", 
            //h1 { "Galleria Dinamica Casabaldini" }
            
            //p { 
               // if cfg!(target_arch = "wasm32") { 
                 //   span { style: "color: green;", "✅ CLIENT ATTIVO" }
                //} else { 
                //    span { style: "color: orange;", "🏠 SERVER RENDERING" }
                //}
            //}

            hr {}
            ElencoSliders {}
        }
    }
}