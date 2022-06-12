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

impl Time {
    fn new(hours: u8, minutes: u8) -> Result<Self, TimesTableError> {
        if hours > 23 || minutes > 59 {
            Err(TimesTableError::InvalidTime)
        } else {
            Ok(Self { hours, minutes })
        }
    }

    fn from_time_input_value(value: &str) -> Option<Self> {
        let mut splits = value.split(':');
        let hours = splits.next()?.parse::<u8>().ok()?;
        let minutes = splits.next()?.parse::<u8>().ok()?;
        Some(Self { hours, minutes })
    }
}

#[derive(Debug)]
enum TimesTableError {
    InvalidTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_works() -> Result<(), TimesTableError> {
        new_with_valid_input(5, 30)?;
        new_with_valid_input(10, 20)?;
        new_with_valid_input(13, 54)?;
        new_with_valid_input(19, 27)?;
        Ok(())
    }

    fn new_with_valid_input(hours: u8, minutes: u8) -> Result<(), TimesTableError> {
        let t = Time::new(hours, minutes)?;
        assert_eq!(t.hours, hours);
        assert_eq!(t.minutes, minutes);
        Ok(())
    }

    #[test]
    fn parses_hh_mm() -> Result<(), TimesTableError> {
        // Normal input
        parses_valid_hh_mm("05:30", 5, 30);
        parses_valid_hh_mm("10:20", 10, 20);
        parses_valid_hh_mm("13:54", 13, 54);
        parses_valid_hh_mm("19:27", 19, 27);

        Ok(())
    }

    fn parses_valid_hh_mm(value: &str, hours: u8, minutes: u8) {
        let t = Time::from_time_input_value(value).unwrap();
        assert_eq!(t, Time::new(hours, minutes).expect("could not create Time"));
    }
}
