#![allow(unused)]

use gloo::console::log;
use stylist::{style, yew::styled_component};
use yew::prelude::*;

mod components;

use components::atoms::main_title::MainTitle;

#[styled_component(App)]
pub fn app() -> Html {
    let my_style = style!(
        r#"
        color: gold;
        font-family: monospace;
        font-size: 64px;
        "#
    ).unwrap();

    html! {
        <div class={my_style}>
            <MainTitle title="Test"/>
        </div>
    }
}
