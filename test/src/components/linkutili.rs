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
        a{ style:"color:#ffffff; font-size: 1em;  font-weight: bold;", href:"{l.link}", img{ src:"/assets/img/links/{l.img}", width:"1%", height:"1%", alt:""},  "  ",span{style:"color:#ffffff; font-size: 1.5em; font-weight: bold; width:1%; height:1%; margin-left:12%",FastImage { name: l.img.clone(), dir:{"links"} }},"{l.titolo}" }
		    }
      }
    } 
    },
     _ => rsx! { img { src: CLESSIDRA, id: "header" } }
  }
}
}

	



