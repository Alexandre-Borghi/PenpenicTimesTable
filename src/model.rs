use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TimesTableState {
    days: [Day; 6],
    show_saturday: bool,
}

impl TimesTableState {
    pub fn from_local_storage() -> Option<Self> {
        // todo!("read the state from localStorage");
        None
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
struct Day {
    morning: TimesRange,
    afternoon: TimesRange,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
struct TimesRange {
    start: Option<Time>,
    end: Option<Time>,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
struct Time {
    hours: u8,
    minutes: u8,
}
