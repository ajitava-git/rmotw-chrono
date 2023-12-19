extern crate chrono;

use chrono::prelude::*;
use chrono::offset::{TimeZone, Local, FixedOffset};
use chrono::Duration;


// Define a struct for TimeZoneConverter
struct TimeZoneConverter;

impl TimeZoneConverter {
    // Function to convert a DateTime to a different time zone
    fn convert_to_timezone<Tz: TimeZone>(&self, dt: DateTime<Utc>, timezone: Tz) -> DateTime<Tz> {
        timezone.from_utc_datetime(&dt.naive_utc())
    }

    // Function to calculate the time difference between two time zones
    fn time_difference<Tz1: TimeZone, Tz2: TimeZone>(&self, tz1: Tz1, tz2: Tz2) -> Duration {
        let now_utc = Utc::now();
        let now_tz1 = self.convert_to_timezone(now_utc, tz1);
        let now_tz2 = self.convert_to_timezone(now_utc, tz2);

        now_tz1.signed_duration_since(now_tz2)
    }
}

fn main() {
    // Create an instance of TimeZoneConverter
    let converter = TimeZoneConverter;

    // Define two time zones: Local and New York
    let local_timezone = Local;
    let ny_timezone = FixedOffset::west_opt(5 * 3600).expect("Invalid time zone offset");

    // Calculate and print the time difference between Local and New York
    let time_difference = converter.time_difference(local_timezone, ny_timezone);
    println!("Time difference between Local and New York: {}", time_difference);

    // Create a vector of events in UTC
    let events_utc = vec![
        Utc::now() + Duration::hours(1),
        Utc::now() + Duration::hours(5),
        Utc::now() + Duration::hours(10),
    ];

    // Convert and print events in Local time zone
    let events_local: Vec<_> = events_utc
        .iter()
        .map(|&dt| converter.convert_to_timezone(dt, local_timezone))
        .collect();

    println!("Events in Local time zone: {:?}", events_local);

    // Convert and print events in New York time zone
    let events_ny: Vec<_> = events_utc
        .iter()
        .map(|&dt| converter.convert_to_timezone(dt, ny_timezone))
        .collect();

    println!("Events in New York time zone: {:?}", events_ny);
}
