use chrono::{DateTime,Utc,Local,FixedOffset};

pub fn date_time() {
    let utc_time: DateTime<Utc> = Utc::now();
    let local_time: DateTime<Local> = Local::now();
    let fixed_offset = FixedOffset::east_opt(5 * 3600);
    println!("{:?}", fixed_offset);
    println!("{:?}", utc_time);
    println!("{:?}", local_time);
}

