use yew::function_component;
use yew::prelude::*;

#[function_component(TimesTableRowRange)]
pub fn times_table_row_range() -> Html {
    let start_input_ref = use_node_ref();
    let end_input_ref = use_node_ref();

    html! {
        <>
            <div class={classes!("col")}>
                <input ref={start_input_ref} type="time" class={classes!("form-control")} />
            </div>
            <div class={classes!("col")}>
                <input ref={end_input_ref} type="time" class={classes!("form-control")} />
            </div>
        </>
    }
}
