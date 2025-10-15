use dioxus::{html::h1, prelude::*};

#[component]
pub fn Main() -> Element {
    rsx! {
        div {
            id: "title",
            h1 { "FLXOR" }
        },
        div { 
            id: "body"
         }
    }
}