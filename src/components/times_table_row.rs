use yew::function_component;
use yew::prelude::*;

use super::times_table_row_range::TimesTableRowRange;

#[function_component(TimesTableRow)]
pub fn times_table_row() -> Html {
    html! {
        <div class={classes!("row")}>
            <TimesTableRowRange></TimesTableRowRange>
            <TimesTableRowRange></TimesTableRowRange>
        </div>
    }
}
