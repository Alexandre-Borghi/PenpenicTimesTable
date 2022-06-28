use gloo_console::log;
use yew::function_component;
use yew::prelude::*;

use crate::model::Day;
use crate::model::TimesRange;

use super::times_table_row_range::TimesTableRowRange;

#[derive(PartialEq, Properties, Clone)]
pub struct TimesTableRowProps {
    pub day: Day,
}

#[function_component(TimesTableRow)]
pub fn times_table_row(props: &TimesTableRowProps) -> Html {
    enum Period {
        Morning,
        Afternoon,
    }

    let on_change = |times_range: TimesRange, period: Period| {
        match period {
            Period::Morning => log!(format!("New morning: {:?}", times_range)),
            Period::Afternoon => log!(format!("New afternoon: {:?}", times_range)),
        };
    };

    let on_change_morning =
        Callback::from(move |times_range: TimesRange| on_change(times_range, Period::Morning));
    let on_change_afternoon =
        Callback::from(move |times_range: TimesRange| on_change(times_range, Period::Afternoon));

    html! {
        <div class={classes!("row")}>
            // TODO: Remove clones
            <TimesTableRowRange times_range={props.day.morning.clone()} on_change={on_change_morning}></TimesTableRowRange>
            <TimesTableRowRange times_range={props.day.afternoon.clone()} on_change={on_change_afternoon}></TimesTableRowRange>

            <div class={classes!("col", "col-1")}>
                <input type="time" class={classes!("form-control")} readonly=true />
            </div>
        </div>
    }
}
