use crate::prelude::*;
//use dioxus::prelude::*;
///use crate::models::{Links};
use crate::components::fastimage::FastImage;
use dioxus::prelude::asset;
use crate::models::get_link_db;
#[component]
pub fn Linkutili() -> Element {
    let link_res = use_resource(move || get_link_db());
    
rsx! {
  match &*link_res.read_unchecked() {
            Some(Ok(lista)) => rsx! {
    div{ class:"marquee",
      div{ class:"marquee-content",
    
        for l in lista {
        //div{ FastImage { name: l.img.clone() }}
				a{ style:"color:#ffffff; font-size: 1.5em; margin-left:2%; margin-top:2%; font-weight: bold;", href:"{l.link}", img{ src:"/assets/img/links/{l.img}", width:"10%", height:"10%", alt:""},  "  ", "{l.img}","{l.titolo}" }
         //a { href: //"https://dioxuslabs.com/learn/0.7/", "📚 Learn Dioxus" }
         //a{ href:"{l.link}", img{ src:"/assets/img/links/{l.img}", width:"10%", height:"10%", alt:""},  "  ", "{l.titolo}" } 
		    }
      }
    } 
    },
     _ => rsx! { p { "Caricamento in corso..." } }
  }
}
}

	



