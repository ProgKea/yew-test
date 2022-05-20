use gloo::console::log;
use rand::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub count: String,
}

fn random_words(count: i32) -> Vec<String> {
    let mut random_words: Vec<String> = include_str!("1-1000.txt")
        .split_whitespace()
        .map(|s| s.to_string() + " ")
        .collect();

    let mut rng = rand::thread_rng();
    random_words.shuffle(&mut rng);

    random_words[0..=count as usize].to_vec()
}

#[function_component(RandomWords)]
pub fn random_words_html(props: &Props) -> Html {
    let random_words = random_words(props.count.parse::<i32>().unwrap_or(1));

    html! {
        <p>{random_words}</p>
    }
}
