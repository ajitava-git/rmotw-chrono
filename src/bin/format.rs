use chrono::format::{ParseError, StrftimeItems};
use chrono::{DateTime, Local};

struct NewDateTime {
    datetime: DateTime<Local>,
}

impl NewDateTime {
    fn new(datetime_str: &str, format_str: &str) -> Result<Self, ParseError> {
        let datetime = DateTime::parse_from_str(datetime_str, format_str)?;
        Ok(Self { datetime: datetime.into() })
    }

    fn format_with_items(&self, items: StrftimeItems) -> String {
        self.datetime.format_with_items(items).to_string()
    }
}

fn main() {
    // Custom date and time format
    let custom_format = "%Y-%m-%d %H:%M:%S %:z";
    
    // Date time string in custom format with timezone 
    let date_time_string = "2024-01-01 02:30:00 +05:30";

    match NewDateTime::new(date_time_string, custom_format) {
        Ok(my_datetime) => {
            // Formating the parsed DateTime object into a different format using StrftimeItems submodule
            let new_format_items = StrftimeItems::new("%A, %d %B %Y %H:%M:%S");
            let formatted_datetime = my_datetime.format_with_items(new_format_items);

            println!("Parsed and formatted date and time: {}", formatted_datetime);
        }
        Err(err) => {
            eprintln!("Error parsing date and time: {}", err);
        }
    }
}
