use yew::function_component;
use yew::prelude::*;

mod components;
use components::TimesTable;

mod model;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "Tableau d'heures v3" }</h1>
            <TimesTable></TimesTable>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
