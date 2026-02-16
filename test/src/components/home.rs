/// Home page
pub use dioxus::prelude::*;
//pub use components::echo::Echo;
pub use crate::components::hero::Hero;
use crate::components::echo::Echo;
#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        Echo {}
    }
}