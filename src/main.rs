use yew::function_component;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>{ "Hey" }</div>
    }
}

fn main() {
    yew::start_app::<App>();
}
