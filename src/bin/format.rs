use chrono::format::StrftimeItems;
use chrono::DateTime;

fn main() {
    // Define a custom date and time format
    let custom_format = "%Y-%m-%d %H:%M:%S %:z";
    
    // A sample date and time string in the custom format with time zone information
    let date_time_string = "2023-12-20 02:30:00 +05:00";
    
    // Parse the date and time string into a DateTime object
    let parsed_datetime = DateTime::parse_from_str(date_time_string, custom_format);

    match parsed_datetime {
        Ok(dt) => {
            // Format the parsed DateTime object into a different format
            let new_format = StrftimeItems::new("%A, %d %B %Y %H:%M:%S");
            let formatted_datetime = dt.format_with_items(new_format);

            println!("Parsed and formatted date and time: {}", formatted_datetime);
        }
        Err(err) => {
            eprintln!("Error parsing date and time: {}", err);
        }
    }
}