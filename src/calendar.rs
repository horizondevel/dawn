use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Day {
    pub month: Month,
    pub day: u8,
    pub week_day: WeekDay,
}
