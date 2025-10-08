use chrono::{Datelike, Duration, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Calendar {
    pub month: Month,
    pub year: i32,
    pub days: Vec<Day>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}
impl Month {
    pub const fn from_chrono(month_index: u32) -> Self {
        match month_index {
            1 => Self::January,
            2 => Self::February,
            3 => Self::March,
            4 => Self::April,
            5 => Self::May,
            6 => Self::June,
            7 => Self::July,
            8 => Self::August,
            9 => Self::September,
            10 => Self::October,
            11 => Self::November,
            12 => Self::December,
            _ => panic!("Invalid month index"),
        }
    }
    pub const fn get_index(month: &Month) -> u32 {
        let chrono_month: u32 = match month {
            Month::January => 1,
            Month::February => 2,
            Month::March => 3,
            Month::April => 4,
            Month::May => 5,
            Month::June => 6,
            Month::July => 7,
            Month::August => 8,
            Month::September => 9,
            Month::October => 10,
            Month::November => 11,
            Month::December => 12,
        };
        chrono_month
    }
}
impl FromStr for Month {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "January" => Ok(Month::January),
            "February" => Ok(Month::February),
            "March" => Ok(Month::March),
            "April" => Ok(Month::April),
            "May" => Ok(Month::May),
            "June" => Ok(Month::June),
            "July" => Ok(Month::July),
            "August" => Ok(Month::August),
            "September" => Ok(Month::September),
            "October" => Ok(Month::October),
            "November" => Ok(Month::November),
            "December" => Ok(Month::December),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WeekDay {
    Mon,
    Tues,
    Wed,
    Thurs,
    Fri,
    Sat,
    Sun,
}

impl WeekDay {
    fn from_chrono(chrono_weekday: Weekday) -> WeekDay {
        match chrono_weekday {
            Weekday::Mon => WeekDay::Mon,
            Weekday::Tue => WeekDay::Tues,
            Weekday::Wed => WeekDay::Wed,
            Weekday::Thu => WeekDay::Thurs,
            Weekday::Fri => WeekDay::Fri,
            Weekday::Sat => WeekDay::Sat,
            Weekday::Sun => WeekDay::Sun,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Day {
    pub month: Month,
    pub day: u32,
    pub week_day: WeekDay,
}

pub fn make_calendar(month: Month, year: i32) -> Calendar {
    let chrono_month: u32 = Month::get_index(&month);

    let mut first_of_the_calendar = NaiveDate::from_ymd_opt(year, chrono_month, 1).unwrap();
    while first_of_the_calendar.weekday() != Weekday::Sun {
        first_of_the_calendar -= Duration::days(1)
    }

    let mut days: Vec<Day> = Vec::new();
    for d in first_of_the_calendar.iter_days() {
        days.push(Day {
            month: Month::from_chrono(d.month()),
            day: d.day(),
            week_day: WeekDay::from_chrono(d.weekday()),
        });
        if d.weekday() == Weekday::Sat && d.month() > chrono_month {
            break;
        }
    }

    Calendar { year, month, days }
}
