// use std::time::Duration;

use time::{Duration, PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    // todo!("What time is a gigasecond later than {start}");
    let giga_second = Duration::new(1000000000, 0);
    let updated_start = start.checked_add(giga_second).unwrap();
    updated_start
}
