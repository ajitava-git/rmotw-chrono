// This script demonstrate working with events and date/time calculations using the 'prelude' module.

// It provides methods to create a new event, calculate the number of days until the event occurs, and format the event timestamp.
// The main function creates an event, prints its details, calculates the days until the event
// occurs from the current date, and formats the event timestamp.

use chrono::prelude::{Local, TimeZone, DateTime};

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
    let event_timestamp = Local.ymd(2024, 2, 15).and_hms(14, 30, 0); 
    let event = Event::new(event_name, event_timestamp);

    // Print event details
    println!("Event: {} at {}", event.name, event.timestamp);

    // Calculate days until the event
    let today = Local::now(); 
    let days_until_event = event.days_until(today); 
    println!("Days until {}: {}", event.name, days_until_event);

    // Format the event timestamp
    let formatted_timestamp = event.timestamp.format("%Y-%m-%d %H:%M:%S");
    println!("Formatted timestamp: {}", formatted_timestamp);
}
