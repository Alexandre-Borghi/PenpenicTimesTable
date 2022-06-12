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
        new_with_valid_input(10, 20)?;
        new_with_valid_input(5, 30)?;
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
        assert_eq!(
            Time::from_time_input_value("05:30").unwrap(),
            Time::new(5, 30)?
        );

        Ok(())
    }
}
