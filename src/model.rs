use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TimesTableState {
    pub days: [Day; 6],
    pub show_saturday: bool,
}

impl TimesTableState {
    pub fn from_local_storage() -> Option<Self> {
        // todo!("read the state from localStorage");
        None
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Day {
    pub morning: TimesRange,
    pub afternoon: TimesRange,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimesRange {
    pub start: Option<Time>,
    pub end: Option<Time>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Time {
    hours: u8,
    minutes: u8,
}

impl Time {
    pub fn new(hours: u8, minutes: u8) -> Result<Self, TimesTableError> {
        if hours > 23 || minutes > 59 {
            Err(TimesTableError::InvalidTime)
        } else {
            Ok(Self { hours, minutes })
        }
    }

    pub fn from_time_input_value(value: &str) -> Option<Self> {
        let mut splits = value.split(':');
        let hours = splits.next()?.parse::<u8>().ok()?;
        let minutes = splits.next()?.parse::<u8>().ok()?;
        let time = Self::new(hours, minutes);
        if let Ok(time) = time {
            Some(time)
        } else {
            None
        }
    }

    pub fn to_time_input_value(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }

    pub fn option_to_input_value(opt: &Option<Self>) -> String {
        match opt {
            None => "".to_string(),
            Some(time) => time.to_time_input_value(),
        }
    }
}

#[derive(Debug)]
pub enum TimesTableError {
    InvalidTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_works() -> Result<(), TimesTableError> {
        // Normal input
        new_with_valid_input(5, 30)?;
        new_with_valid_input(10, 20)?;
        new_with_valid_input(13, 54)?;
        new_with_valid_input(19, 27)?;

        // Edge cases
        new_with_valid_input(0, 0)?;
        new_with_valid_input(23, 59)?;

        // Bad input
        new_with_bad_input(30, 33);
        new_with_bad_input(24, 20);
        new_with_bad_input(15, 123);
        new_with_bad_input(15, 60);

        Ok(())
    }

    fn new_with_valid_input(hours: u8, minutes: u8) -> Result<(), TimesTableError> {
        let t = Time::new(hours, minutes)?;
        assert_eq!(t.hours, hours);
        assert_eq!(t.minutes, minutes);
        Ok(())
    }

    fn new_with_bad_input(hours: u8, minutes: u8) {
        if Time::new(hours, minutes).is_ok() {
            panic!("Time::new succeeds on bad input");
        }
    }

    #[test]
    fn time_parses_hh_mm() -> Result<(), TimesTableError> {
        // Normal input
        parses_valid_time_input_value("05:30", 5, 30);
        parses_valid_time_input_value("10:20", 10, 20);
        parses_valid_time_input_value("13:54", 13, 54);
        parses_valid_time_input_value("19:27", 19, 27);

        // Edge cases
        parses_valid_time_input_value("00:00", 0, 0);
        parses_valid_time_input_value("12:00", 12, 0);
        parses_valid_time_input_value("23:59", 23, 59);

        Ok(())
    }

    #[test]
    fn time_parses_hh_mm_ss() -> Result<(), TimesTableError> {
        // Normal input
        parses_valid_time_input_value("05:30:56", 5, 30);
        parses_valid_time_input_value("10:20:22", 10, 20);
        parses_valid_time_input_value("13:54:47", 13, 54);
        parses_valid_time_input_value("19:27:33", 19, 27);

        // Edge cases
        parses_valid_time_input_value("00:00:00", 0, 0);
        parses_valid_time_input_value("12:00:00", 12, 0);
        parses_valid_time_input_value("23:59:59", 23, 59);

        Ok(())
    }

    #[test]
    fn time_doesnt_parse_bad_input() {
        doesnt_parse_bad_input("");
        doesnt_parse_bad_input("bad input");
        doesnt_parse_bad_input("-10:20");
        doesnt_parse_bad_input("30:20");
        doesnt_parse_bad_input("24:20");
        doesnt_parse_bad_input("10:123");
        doesnt_parse_bad_input("12:60");
    }

    fn parses_valid_time_input_value(value: &str, hours: u8, minutes: u8) {
        let t = Time::from_time_input_value(value).unwrap();
        assert_eq!(t, Time::new(hours, minutes).expect("could not create Time"));
    }

    fn doesnt_parse_bad_input(value: &str) {
        if Time::from_time_input_value(value).is_some() {
            panic!("Time::from_time_input_value succeeds with bad input");
        }
    }

    #[test]
    fn time_to_input_value() {
        test_case_time_to_input_value(Time::new(0, 0).unwrap(), "00:00");
        test_case_time_to_input_value(Time::new(7, 55).unwrap(), "07:55");
        test_case_time_to_input_value(Time::new(10, 30).unwrap(), "10:30");
        test_case_time_to_input_value(Time::new(23, 59).unwrap(), "23:59");
    }

    fn test_case_time_to_input_value(t: Time, expected: &str) {
        assert_eq!(t.to_time_input_value(), expected);
    }

    #[test]
    fn time_option_to_input_value() {
        assert_eq!(Time::option_to_input_value(&None), "");
        assert_eq!(
            Time::option_to_input_value(&Some(Time::new(0, 0).unwrap())),
            "00:00"
        );
        assert_eq!(
            Time::option_to_input_value(&Some(Time::new(7, 55).unwrap())),
            "07:55"
        );
        assert_eq!(
            Time::option_to_input_value(&Some(Time::new(23, 59).unwrap())),
            "23:59"
        );
    }
}
