use yew::function_component;
use yew::prelude::*;

#[function_component(TimesTableRowRange)]
pub fn times_table_row_range() -> Html {
    html! {
        <>
            <div class={classes!("col")}>
                <input type="time" />
            </div>
            <div class={classes!("col")}>
                <input type="time" />
            </div>
        </>
    }
}
