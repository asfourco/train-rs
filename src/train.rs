use anyhow::{Context, Result};
use chrono::NaiveTime;
use requestty::{self, Question};

use crate::actions::{add_remove_prompt, Action};
use crate::helper::clear_screen;

pub struct Train {
    line: u32,
    name: String,
    capacity: u32,
    origin: String,
    destination: String,
    departure: NaiveTime,
    arrival: NaiveTime,
}

impl Train {
    fn new(
        line: u32,
        name: String,
        capacity: u32,
        origin: String,
        destination: String,
        departure: NaiveTime,
        arrival: NaiveTime,
    ) -> Self {
        Self {
            line,
            name,
            capacity,
            origin,
            destination,
            departure,
            arrival,
        }
    }
}

pub fn manage_trains(trains: &mut Vec<Train>) -> Result<()> {
    loop {
        clear_screen();
        match add_remove_prompt()? {
            Action::List => {
                if trains.is_empty() {
                    println!("No trains found");
                } else {
                    for train in trains.iter() {
                        println!(
                            "Line: {}\nName: {}\nCapacity: {}\nOrigin: {}\nDestination: {}\nDeparture: {}\nArrival: {}\n====================\n",
                            train.line, train.name, train.capacity, train.origin, train.destination, train.departure, train.arrival
                        );
                    }
                }
                // Prompt to continue
                requestty::prompt_one(
                    Question::input("continue")
                        .message("Press Enter to continue")
                        .build(),
                )?;
            }
            Action::Add => {
                println!("You selected Add");
                trains.push(add_train()?);
            }
            Action::Remove => {
                println!("You selected Remove");
            }
            Action::Back => {
                println!("Returning to Main Menu");
                return Ok(());
            }
        }
    }
}

fn add_train() -> Result<Train> {
    let questions: Vec<Question> = vec![
        Question::input("line")
            .message("Enter the line number of the train")
            .build(),
        Question::input("name")
            .message("Enter the name of the train")
            .build(),
        Question::input("capacity")
            .message("Enter the capacity of the train")
            .build(),
        Question::input("origin")
            .message("Enter the origin of the train")
            .build(),
        Question::input("destination")
            .message("Enter the destination of the train")
            .build(),
        Question::input("departure")
            .message("Enter the departure time of the train (HH:MM)")
            .build(),
        Question::input("arrival")
            .message("Enter the arrival time of the train (HH:MM)")
            .build(),
    ];

    let train = requestty::prompt(questions)?;

    let departure_time = NaiveTime::parse_from_str(
        train.get("departure").unwrap().as_string().unwrap(),
        "%H:%M",
    )
    .context("Invalid departure time")?;

    let arrival_time =
        NaiveTime::parse_from_str(train.get("arrival").unwrap().as_string().unwrap(), "%H:%M")
            .context("Invalid arrival time")?;

    Ok(Train::new(
        train
            .get("line")
            .unwrap()
            .as_string()
            .unwrap()
            .parse()
            .unwrap(),
        train.get("name").unwrap().as_string().unwrap().to_string(),
        train
            .get("capacity")
            .unwrap()
            .as_string()
            .unwrap()
            .parse()
            .unwrap(),
        train
            .get("origin")
            .unwrap()
            .as_string()
            .unwrap()
            .to_string(),
        train
            .get("destination")
            .unwrap()
            .as_string()
            .unwrap()
            .to_string(),
        departure_time,
        arrival_time,
    ))
}
