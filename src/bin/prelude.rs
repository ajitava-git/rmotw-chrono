extern crate chrono;

use chrono::prelude::*;

struct Event {
    name: String,
    timestamp: DateTime<Local>,
}

impl Event {
    fn new(name: &str, timestamp: DateTime<Local>) -> Self {
        Event {
            name: name.to_string(),
            timestamp,
        }
    }

    fn days_until(&self, other: DateTime<Local>) -> i64 {
        let duration = other.signed_duration_since(self.timestamp);
        duration.num_days()
    }
}

fn main() {
    // Create an event
    let event_name = "Meeting";
    let event_timestamp = Local.ymd(2023, 12, 15).and_hms(14, 30, 0);
    let event = Event::new(event_name, event_timestamp);

    // Print event details
    println!("Event: {} at {}", event.name, event.timestamp);

    // Calculate days until the event
    let today = Local::today().and_hms(12, 0, 0);
    let days_until_event = event.days_until(today);
    println!("Days until {}: {}", event.name, days_until_event);

    // Format the event timestamp
    let formatted_timestamp = event.timestamp.format("%Y-%m-%d %H:%M:%S");
    println!("Formatted timestamp: {}", formatted_timestamp);
}






// use chrono::format::StrftimeItems;

// fn main() {
//     // Define a custom date and time format
//     let custom_format = "%Y-%m-%d %H:%M:%S %:z";
    
//     // A sample date and time string in the custom format with time zone information
//     let date_time_string = "2023-11-03 30:30:00 +05:00";
    
//     // Parse the date and time string into a DateTime object
//     let parsed_datetime = DateTime::parse_from_str(date_time_string, custom_format);

//     match parsed_datetime {
//         Ok(dt) => {
//             // Format the parsed DateTime object into a different format
//             let new_format = StrftimeItems::new("%A, %d %B %Y %H:%M:%S");
//             let formatted_datetime = dt.format_with_items(new_format);

//             println!("Parsed and formatted date and time: {}", formatted_datetime);
//         }
//         Err(err) => {
//             eprintln!("Error parsing date and time: {}", err);
//         }
//     }
// }
