use serde::{Serialize, Deserialize};
use dioxus::{fullstack::reqwest::Url, prelude::*};
#[cfg(feature = "server")]
use sqlx::{PgPool, FromRow};
#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)] 
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct Menus {
	pub id:       i64,
	pub codice:   String,
	pub radice:   String,
	pub livello:  i64,
	pub titolo:   String,
	pub link:     String,
    pub ordine:   i64,
	
}
const DB_URL: &str = "postgres://carlo:treX39@57.131.31.228:5432/casabaldini";
#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)] 
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct Submenus{
	pub id:       i64,
	pub codice:   String,
	pub radice:   String,
	pub livello:  i64,
	pub titolo:   String,
	pub link:     String,
    pub ordine:   i64,
	
}

#[server]
pub async fn get_menu_db() -> Result<Vec<Menus>, ServerFnError> {
    // Trasformiamo l'errore di connessione e di query in stringhe leggibili da ServerFnError
    let pool = PgPool::connect(DB_URL)
        .await
        .map_err(|e| ServerFnError::new(format!("Errore connessione DB: {}", e)))?;

    let mrows = sqlx::query_as::<_, Menus>("SELECT id, codice,  radice,livello,titolo,link, ordine FROM menu where livello=2 and attivo= 1 order by ordine")
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Errore query: {}", e)))?;
    println!("📡 Server: Row recuperate, invio in corso...");
    Ok(mrows)
}

#[server]
pub async fn get_submenu_db() -> Result<Vec<Submenus>, ServerFnError> {
    // Trasformiamo l'errore di connessione e di query in stringhe leggibili da ServerFnError
    let pools = PgPool::connect(DB_URL)
        .await
        .map_err(|e| ServerFnError::new(format!("Errore connessione DB: {}", e)))?;

    let srows = sqlx::query_as::<_, Submenus>("SELECT id, codice,  radice, livello, titolo,link, ordine FROM submenu where attivo = 1 order by ordine")
        .fetch_all(&pools)
        .await
        .map_err(|e| ServerFnError::new(format!("Errore query: {}", e)))?;
    println!("📡 Server: Row recuperate, invio in corso...");
    Ok(srows)
}