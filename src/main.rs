use time::PrimitiveDateTime as DateTime;
// use time_macros::{date, datetime, time};
use time::{Date, Month, Time};
fn main() {
    let date: Date = Date::from_calendar_date(2024, Month::January, 21).unwrap();
    let time: Time = Time::from_hms(17, 01, 00).unwrap();
    let start: DateTime = DateTime::new(date, time);
    gigasecond::after(start);
}
