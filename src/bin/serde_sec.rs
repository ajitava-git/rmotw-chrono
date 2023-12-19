use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct SecStruct {
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
    pub message: String,
}

impl SecStruct {
    pub fn new(message: &str) -> Self {
        SecStruct {
            timestamp: Utc::now(),
            message: message.to_string(),
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

fn main() {
    // Create a new instance of SecStruct
    let sec_struct = SecStruct::new("Hello!");

    // Convert the struct to JSON
    let json_result = sec_struct.to_json();
    match json_result {
        Ok(json) => {
            println!("JSON representation: {}", json);

            // Parse the JSON back into a SecStruct
            let parsed_result = SecStruct::from_json(&json);
            match parsed_result {
                Ok(parsed_struct) => {
                    println!("Parsed struct: {:?}", parsed_struct);
                }
                Err(err) => {
                    eprintln!("Error parsing JSON: {}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("Error converting struct to JSON: {}", err);
        }
    }
}
