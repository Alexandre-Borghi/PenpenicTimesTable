use crate::model::{Time, TimesRange};
use web_sys::HtmlInputElement;
use yew::function_component;
use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct TimesTableRowRangeProps {
    pub times_range: TimesRange,
    pub on_change: Callback<TimesRange>,
}

#[function_component(TimesTableRowRange)]
pub fn times_table_row_range(props: &TimesTableRowRangeProps) -> Html {
    let start_input_ref = use_node_ref();
    let end_input_ref = use_node_ref();

    let initial_start = Time::option_to_input_value(&props.times_range.start);
    let initial_end = Time::option_to_input_value(&props.times_range.end);

    let oninput = {
        let start_input_ref = start_input_ref.clone();
        let end_input_ref = end_input_ref.clone();
        let on_change = props.on_change.clone();

        Callback::from(move |_: InputEvent| {
            let start_input = start_input_ref
                .cast::<HtmlInputElement>()
                .expect("could not find start input");
            let end_input = end_input_ref
                .cast::<HtmlInputElement>()
                .expect("could not find end input");

            let times_range = TimesRange {
                start: Time::from_time_input_value(&start_input.value()),
                end: Time::from_time_input_value(&end_input.value()),
            };
            on_change.emit(times_range);
        })
    };

    html! {
        <>
            <div class={classes!("col")}>
                <input ref={start_input_ref} oninput={oninput.clone()} type="time" value={initial_start} class={classes!("form-control")} />
                // <div class={classes!("form-text")}>{ "Début" }</div>
            </div>
            <div class={classes!("col")}>
                <input ref={end_input_ref} oninput={oninput.clone()} type="time" value={initial_end} class={classes!("form-control")} />
                // <div class={classes!("form-text")}>{ "Fin" }</div>
            </div>
        </>
    }
}
