#![allow(unused)]

use gloo::console::log;
use stylist::{style, yew::styled_component};
use yew::prelude::*;

mod components;

use components::atoms::main_title::MainTitle;
use components::atoms::random_words::RandomWords;

#[styled_component(App)]
pub fn app() -> Html {
    let title_style = style!(
        r#"
        color: gold;
        text-align: center;
        font-size: 64px;
        "#
    ).unwrap();

    html! {
        <div class={title_style}>
            <MainTitle title="Gorillatest"/>
        </div>
    }
}
