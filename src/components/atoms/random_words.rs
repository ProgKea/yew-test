use rand::prelude::*;
use stylist::{style, yew::styled_component};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub count: String,
    pub handle_onkeyup: Callback<String>,
    pub handle_onfocus: Callback<Option<Vec<String>>>,
    pub handle_onfocusout: Callback<()>,
}

fn random_words(count: i32) -> Vec<String> {
    let mut random_words: Vec<String> = include_str!("1-1000.txt")
        .split_whitespace()
        .map(|s| s.to_string() + " ")
        .collect();

    let mut rng = rand::thread_rng();
    random_words.shuffle(&mut rng);

    if count > 999 {
        random_words
    } else {
        random_words[0..=count as usize].to_vec()
    }
}

#[styled_component(RandomWords)]
pub fn random_words_html(props: &Props) -> Html {
    let random_words = random_words(props.count.parse::<i32>().unwrap_or(1));
    let random_words_state = use_state_eq(|| random_words.clone());
    let stylesheet = style!(
        r#"
        color: white;
        font-size: 75%;
        input {
            z-index: 10;
            cursor: pointer;
            position: sticky;
            width: 75%;
            height: 100px;
            opacity: 0%;
        }
        "#
    )
    .unwrap();

    let handle_onkeyup = props.handle_onkeyup.clone();
    let onkeyup = Callback::from(move |event: KeyboardEvent| {
        handle_onkeyup.emit(
            event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value(),
        );
    });

    let handle_onfocus = props.handle_onfocus.clone();
    let onfocus = Callback::from(move |_event: FocusEvent| {
        handle_onfocus.emit(Some(random_words_state.to_vec()));
    });
    let handle_onfocusout = props.handle_onfocusout.clone();
    let onfocusout = Callback::from(move |_event: FocusEvent| {
        handle_onfocusout.emit(());
    });

    html! {
        <>
            <div class={stylesheet}>
                <input onfocusout={onfocusout} onfocus={onfocus} onkeyup={onkeyup}/>
            </div>
        </>
    }
}
