use yew::function_component;
use yew::prelude::*;

use super::times_table_row_range::TimesTableRowRange;
use crate::model::TimesTableState;

#[function_component(TimesTable)]
pub fn times_table() -> Html {
    let state_ref = use_state_eq(|| TimesTableState::from_local_storage().unwrap_or_default());

    html! {
        <form>
            <div class={classes!("row")}>
                <TimesTableRowRange></TimesTableRowRange>
            </div>
        </form>
    }
}
