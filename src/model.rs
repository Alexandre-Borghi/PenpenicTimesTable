#[derive(Debug, Serialize, Deserialize)]
struct TimesTable {
    days: [Day; 6],
    show_saturday: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Day {
    morning: TimesRange,
    afternoon: TimeRange,
}

#[derive(Debug, Serialize, Deserialize)]
struct TimeRange {
    start: Time,
    end: Time,
}

#[derive(Debug, Serialize, Deserialize)]
struct Time {
    hours: u8,
    minutes: u8,
}
