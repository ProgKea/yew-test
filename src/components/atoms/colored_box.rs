use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub color: Color,
    pub text: String,
}

#[derive(PartialEq)]
pub enum Color {
    Green,
    Red,
}

impl Color {
    pub fn to_string(&self) -> String {
        match self {
            Color::Green => "green".to_owned(),
            Color::Red => "red".to_owned(),
        }
    }
}

#[styled_component(ColoredBox)]
pub fn colored_box(props: &Props) -> Html {
    let stylesheet = style!(
        r#"
        .green {
            background-color: Green;
        }

        .red {
            background-color: Red;
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
        }
        "#
    )
    .unwrap();

    html! {
        <div class={stylesheet}>
            <button class={props.color.to_string()}>{&props.text}</button>
        </div>
    }
}
