use crate::components::atoms::colored_box::{Color, ColoredBox};
use yew::prelude::*;

#[function_component(ReactionTest)]
pub fn reaction_test() -> Html {
    let reaction_phase = use_state(|| 0);
    let cloned_reaction_phase = reaction_phase.clone();
    let box_clicked = Callback::from(move |_| {
        cloned_reaction_phase.set(1);
    });

    html! {
        <div>
            if *reaction_phase == 0 {
                <ColoredBox handle_onclick={box_clicked} text="Click to start the test" color={Color::Blue}/>
            }
            else if *reaction_phase == 1 {
                <ColoredBox handle_onclick={box_clicked} text="Click when green" color={Color::Red}/>
            }
            else if *reaction_phase == 2 {
                <ColoredBox handle_onclick={box_clicked} text="" color={Color::Green}/>
            }
        </div>
    }
}
