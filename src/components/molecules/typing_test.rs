use crate::components::atoms::random_words::RandomWords;
use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[styled_component(TypingTest)]
pub fn typing_test() -> Html {
    let input = use_state(|| String::new());
    let cloned_input = input.clone();
    let onkeyup = Callback::from(move |input| {
        cloned_input.set(input);
    });

    let random_words: UseStateHandle<Vec<String>> = use_state(|| Vec::new());
    let cloned_random_words = random_words.clone();

    let focused = use_state(|| false);
    let cloned_focused = focused.clone();
    let cloned_cloned_focused = cloned_focused.clone();
    let onfocus = Callback::from(move |i: Option<Vec<String>>| {
        cloned_focused.set(true);
        cloned_random_words.set(i.unwrap());
    });

    let onfocusout = Callback::from(move |_| {
        cloned_cloned_focused.set(false);
    });

    let stylesheet = style!(
        r#"
        width: 100%;
        height: 100%;
        position: fixed;
        color: #96a6c8;
        "#
    )
    .unwrap();

    html! {
        <>
            if !*focused {
                <div class={stylesheet}>{"Click to start"}</div>
            }
            if random_words.len() > 0 {
                <p>{&*random_words[0]}</p>
            }
            <RandomWords handle_onfocus={onfocus} handle_onfocusout={onfocusout} handle_onkeyup={onkeyup} count="10"/>
        </>
    }
}
