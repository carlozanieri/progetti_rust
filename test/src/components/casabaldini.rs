use crate::prelude::*;
pub use crate::models::get_sliders_db;

use crate::components::elencosliders::ElencoSliders;
#[component]
pub fn Casabaldini(dir: String) -> Element {
    let dir = use_signal(|| dir.to_string());
    let d_resource = dir.clone();
    let d = d_resource.clone();
    let sliders = use_resource(move || get_sliders_db(dir.cloned()));
    rsx! {
        
            div { class:"slider-pro", 
            h1 { "This is dir  {dir}" },
        hr {}
            ElencoSliders {dir}
        }
    }
}