use crate::components::casabaldini::Casabaldini;
/// Home page
pub use crate::prelude::*;
pub use crate::components::hero::Hero;
use crate::components::echo::Echo;
use crate::components::linkutili::Linkutili;
#[component]
pub fn Home() -> Element {
    rsx! {
        Casabaldini {dir:"index"}
        //Echo {}
       // Linkutili {}
    }
}