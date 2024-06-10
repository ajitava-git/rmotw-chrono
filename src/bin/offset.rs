// Timezome converstion script!
// It defines two time zones, `Local` and `New York`, calculates and prints the time difference between them.
// It also creates a vector of events in UTC time zone And converts a vector of events from UTC to both local and New York time zones.

use chrono::offset::FixedOffset;
use chrono::prelude::{Utc, Local, TimeZone};
use chrono::Duration;

fn main() {
    // Define two time zones: Local and New York
    let local_timezone = Local;
    let ny_timezone = match FixedOffset::west_opt(5 * 3600) {
        Some(offset) => offset,
        None => {
            eprintln!("Failed to create New York time zone with west offset");
            match FixedOffset::east_opt(0) {
                Some(offset) => offset,
                None => {
                    eprintln!("Failed to create New York time zone with east offset");
                    let default_offset = match FixedOffset::east_opt(0) {
                        Some(offset) => offset,
                        None => {
                            eprintln!("Failed to create New York time zone with default east offset");
                            panic!("Failed to create New York time zone with any offset");
                        }
                    };
                    default_offset
                }
            }
        }
    };

    // Calculate and print the time difference between Local and New York
    let now_utc = Utc::now();
    let now_local = local_timezone.from_utc_datetime(&now_utc.naive_utc());
    let now_ny = ny_timezone.from_utc_datetime(&now_utc.naive_utc());
    let time_difference = now_local.signed_duration_since(now_ny);
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
        .map(|&dt| local_timezone.from_utc_datetime(&dt.naive_utc()))
        .collect();

    println!("Events in Local time zone: {:?}", events_local);

    // Convert and print events in New York time zone
    let events_ny: Vec<_> = events_utc
        .iter()
        .map(|&dt| ny_timezone.from_utc_datetime(&dt.naive_utc()))
        .collect();

    println!("Events in New York time zone: {:?}", events_ny);
}
