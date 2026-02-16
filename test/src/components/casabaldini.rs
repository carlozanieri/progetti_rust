use crate::prelude::*;
pub use crate::models::get_sliders_db;

use crate::components::elencosliders::ElencoSliders;
#[component]
pub fn Casabaldini() -> Element {
    let sliders = use_resource(move || get_sliders_db());
    rsx! {
        
            div { class:"slider-pro", 
        
        hr {}
            ElencoSliders {}
        }
    }
}