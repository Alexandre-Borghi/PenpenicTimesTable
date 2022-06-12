use yew::function_component;
use yew::prelude::*;

use crate::model::TimesTableState;

#[function_component(TimesTable)]
pub fn times_table() -> Html {
    let state_ref = use_state_eq(|| TimesTableState::from_local_storage().unwrap_or_default());

    html! {
        <div>{ "Times table" }</div>
    }
}