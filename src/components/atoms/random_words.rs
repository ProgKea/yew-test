use gloo::console::log;
use yew::prelude::*;
use reqwest;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub count: String,
}

#[function_component(RandomWords)]
pub fn random_words(props: &Props) -> Html {

    todo!()
}
