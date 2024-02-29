// This program demonstrates parsing a DateTtime string and formatting it using Chrono crate.
// It has two functions:
// - `new_datetime`: Parses a date and time string with a given format and returns a `DateTime<Local>` object.
// - `format_with_items`: Formats a `DateTime<Local>` object using provided format items and returns a string.

use chrono::format::{ParseError, StrftimeItems};
use chrono::{DateTime, Local};

fn new_datetime(datetime_str: &str, format_str: &str) -> Result<DateTime<Local>, ParseError> {
    DateTime::parse_from_str(datetime_str, format_str)
        .map(|datetime| datetime.with_timezone(&Local))
}

fn format_with_items(datetime: DateTime<Local>, items: StrftimeItems) -> String {
    datetime.format_with_items(items).to_string()
}

fn main() {
    // Custom date and time format
    let custom_format = "%Y-%m-%d %H:%M:%S %:z";

    // Date time string in custom format with timezone
    let date_time_string = "2024-01-01 02:30:00 +05:30";

    match new_datetime(date_time_string, custom_format) {
        Ok(parsed_datetime) => {
            // Formatting the parsed DateTime object into a different format using StrftimeItems submodule
            let new_format_items = StrftimeItems::new("%A, %d %B %Y %H:%M:%S");
            let formatted_datetime = format_with_items(parsed_datetime, new_format_items);

            println!("Parsed and formatted date and time: {}", formatted_datetime);
        }
        Err(err) => {
            eprintln!("Error parsing date and time: {}", err);
        }
    }
}
