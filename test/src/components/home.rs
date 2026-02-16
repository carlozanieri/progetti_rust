/// Home page
pub use crate::prelude::*;
pub use crate::components::hero::Hero;
use crate::components::echo::Echo;
#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        Echo {}
    }
}