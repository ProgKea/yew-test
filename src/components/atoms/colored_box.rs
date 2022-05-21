use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub color: Color,
    pub text: String,
    pub handle_onclick: Callback<()>,
}

#[derive(PartialEq)]
pub enum Color {
    Green,
    Red,
    Blue,
}

impl Color {
    pub fn to_string(&self) -> String {
        match self {
            Color::Green => "green".to_owned(),
            Color::Red => "red".to_owned(),
            Color::Blue => "blue".to_owned(),
        }
    }
}

#[styled_component(ColoredBox)]
pub fn colored_box(props: &Props) -> Html {
    let stylesheet = style!(
        r#"
        .green {
            background-color: #73c936;
        }

        .red {
            background-color: #f43841;
        }

        .blue {
            background-color: #1e90ff;
        }

        button {
            border: none;
            color: white;
            padding: 15px 32px;
            text-align: center;
            text-decoration: none;
            display: inline-block;
            font-size: 100%;
            width: 50%;
            height: 264px;
            border-radius: 8px;
        }
        "#
    )
    .unwrap();

    let handle_onclick = props.handle_onclick.clone();

    let onclick = Callback::from(move |_| {
        handle_onclick.emit(());
    });

    html! {
        <div class={stylesheet}>
            <button onclick={onclick} class={props.color.to_string()}>{&props.text}</button>
        </div>
    }
}
