use crate::prelude::*;
pub use crate::models::get_sliders_db;

use crate::components::elencosliders::ElencoSliders;
#[component]
pub fn Casabaldini(dir: String) -> Element {
    // 1. TRUCCO CRUCIALE: Creiamo un segnale che si aggiorna SEMPRE 
    // quando il componente riceve una nuova prop 'dir' dalla rotta.
    // use_memo riesegue la chiusura ogni volta che 'dir' cambia.
    let dir_memo = use_memo(move || dir.clone());

    // 2. Ora la risorsa "osserva" il memo. 
    // Poiché dir_memo() è un segnale reattivo, use_resource ripartirà 
    // ogni volta che il valore del memo cambia.
    let sliders_res = use_resource(move || async move {
        // Leggendo il memo qui dentro, creiamo il legame reattivo
        get_sliders_db(dir_memo()).await
    });

    // 3. Debug: questo apparirà nel terminale ad ogni cambio rotta
    println!("Casabaldini sta visualizzando: {}", dir_memo());

    rsx! {
        div { class: "slider-pro",
            hr {}
            // Passiamo il valore attuale al figlio
            ElencoSliders { dir: dir_memo() }
        }
    }
}