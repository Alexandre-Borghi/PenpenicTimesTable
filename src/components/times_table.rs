use yew::function_component;
use yew::prelude::*;

use super::times_table_row::TimesTableRow;
use crate::model::TimesTableState;

#[function_component(TimesTable)]
pub fn times_table() -> Html {
    let state_ref = use_state_eq(|| TimesTableState::from_local_storage().unwrap_or_default());

    html! {
        <form class={classes!("container")}>
            <TimesTableRow></TimesTableRow>
        </form>
    }
}
