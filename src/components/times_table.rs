use gloo_storage::Storage;
use yew::function_component;
use yew::prelude::*;

use super::times_table_row::TimesTableRow;
use crate::model::TimesTableState;

const STATE_STORAGE_KEY: &str = "v3-state";

#[function_component(TimesTable)]
pub fn times_table() -> Html {
    let state: UseStateHandle<TimesTableState> =
        use_state_eq(|| gloo_storage::LocalStorage::get(STATE_STORAGE_KEY).unwrap_or_default());

    let days_count: usize = {
        let mut count = 5;
        if state.show_saturday {
            count += 1;
        }
        count
    };

    html! {
        <form class={classes!("container")}>
        {
            (0..=days_count).into_iter().map(|i| {
                let state = state.clone();
                html!{
                    // TODO: Remove clone
                    <TimesTableRow key={i} day={state.days[i].clone()}></TimesTableRow>
                }
            }).collect::<Html>()
        }
        </form>
    }
}
