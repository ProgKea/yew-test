#![allow(unused)]

use gloo::console::log;
use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[styled_component(App)]
pub fn app() -> Html {
    let my_style = style!(
        r#"
        color: gold;
        font-family: monospace;
        "#
    ).unwrap();

    html! {
        <>
            <h1 class={my_style}>{"Gorillatype"}</h1>
        </>
    }
}
