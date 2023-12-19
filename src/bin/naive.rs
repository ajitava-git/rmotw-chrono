//Import from the chrono crate
use chrono::naive::NaiveDate;
use chrono::Local;

//Define a struct
struct EventDate {
    event_name: String,
    event_date: NaiveDate,
}

//Implement two methods for EventDate
impl EventDate {

    pub fn new(event_name: &str, year: i32, month: u32, day: u32) -> Result<Self, &'static str> {

        let event_date = NaiveDate::from_ymd_opt(year, month, day);

        match event_date {
            Some(date) => Ok(Self {
                event_name: String::from(event_name),
                event_date: date,
            }),
            None => Err("Invalid date provided"),
        }
    }

    //Check if the event is today
    fn is_today(&self) -> bool {
        let today = Local::now().date_naive();
        self.event_date == today
    }
}

fn main() {

    match EventDate::new("Rust Demo Meeting", 2023, 10, 27) {
        Ok(event) => {

            println!("Event: {}", event.event_name);
            println!("Event Date: {}", event.event_date);


            if event.is_today() {
                println!("This event is scheduled for today!");
            } else {
                println!("This event is not scheduled for today.");
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}