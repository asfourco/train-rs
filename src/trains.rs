use crate::actions::{add_remove_prompt, Action};
use crate::helper::{clear_screen, continue_prompt};
use anyhow::{Context, Result};
use chrono::NaiveTime;
use requestty::{self, Question};
use std::collections::{HashMap, HashSet};

pub struct Train {
    pub line: u32,
    pub name: String,
    pub capacity: u32,
    pub origin: String,
    pub destination: String,
    pub departure: NaiveTime,
    pub arrival: NaiveTime,
    pub passengers: HashSet<String>, // Set of passenger IDs
}

impl Train {
    pub fn new(
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
            passengers: HashSet::new(),
        }
    }
}

pub type TrainList = HashMap<u32, Train>;

pub fn manage_trains(trains: &mut TrainList) -> Result<()> {
    loop {
        clear_screen();
        match add_remove_prompt()? {
            Action::List => {
                if trains.is_empty() {
                    println!("No trains found");
                } else {
                    for (line, train) in &mut *trains {
                        println!(
                            "Line: {}\nName: {}\nCapacity: {}\nOrigin: {}\nDestination: {}\nDeparture: {}\nArrival: {}\n====================\n",
                            line, train.name, train.capacity, train.origin, train.destination, train.departure, train.arrival
                        );
                    }
                }
                continue_prompt();
            }
            Action::Add => {
                let new_train = add_train()?;
                if !trains.contains_key(&new_train.line) {
                    trains.entry(new_train.line).or_insert(new_train);
                } else {
                    println!("Train already exists");
                    continue_prompt();
                }
            }
            Action::Remove => {
                if trains.is_empty() {
                    println!("No trains to remove");
                    continue_prompt();
                } else {
                    remove_train(trains)?;
                }
            }
            Action::Edit => {
                if trains.is_empty() {
                    println!("No trains to edit");
                    continue_prompt();
                } else {
                    edit_train(trains)?;
                }
            }
            Action::Back => {
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
        Question::input("name").message("Enter the name of the train").build(),
        Question::input("origin")
            .message("Enter the origin of the train")
            .build(),
        Question::input("destination")
            .message("Enter the destination of the train")
            .build(),
        Question::input("capacity")
            .message("Enter the capacity of the train")
            .build(),
        Question::input("departure")
            .message("Enter the departure time of the train (HH:MM)")
            .build(),
        Question::input("arrival")
            .message("Enter the arrival time of the train (HH:MM)")
            .build(),
    ];

    let train = requestty::prompt(questions)?;

    let departure_time = NaiveTime::parse_from_str(train.get("departure").unwrap().as_string().unwrap(), "%H:%M")
        .context("Invalid departure time")?;

    let arrival_time = NaiveTime::parse_from_str(train.get("arrival").unwrap().as_string().unwrap(), "%H:%M")
        .context("Invalid arrival time")?;

    Ok(Train::new(
        train.get("line").unwrap().as_string().unwrap().parse()?,
        train.get("name").unwrap().as_string().unwrap().to_string(),
        train.get("capacity").unwrap().as_string().unwrap().parse()?,
        train.get("origin").unwrap().as_string().unwrap().to_string(),
        train.get("destination").unwrap().as_string().unwrap().to_string(),
        departure_time,
        arrival_time,
    ))
}

fn remove_train(trains: &mut TrainList) -> Result<()> {
    let train_lines: Vec<String> = trains.keys().map(|line| line.to_string()).collect();
    let question = Question::select("train_lines")
        .message("Select line to delete")
        .choices(train_lines)
        .build();
    let selection = requestty::prompt_one(question)?;
    let line = selection.as_list_item().unwrap().text.parse::<u32>().unwrap();
    trains.remove(&line);
    Ok(())
}

fn edit_train(trains: &mut TrainList) -> Result<()> {
    let train_choices: Vec<String> = trains
        .values()
        .map(|train| format!("{}, {}", train.line, train.name))
        .collect();
    let question = Question::select("train_list")
        .message("Select train to edit")
        .choices(train_choices)
        .build();
    let selection = requestty::prompt_one(question)?;
    let selected_train = selection.as_list_item().unwrap().text.clone();
    let line = selected_train.split(',').next().unwrap().trim().parse::<u32>().unwrap();

    let questions: Vec<Question> = vec![
        Question::input("name")
            .message("Enter the new name of the train")
            .default(trains[&line].name.clone())
            .build(),
        Question::input("capacity")
            .message("Enter the new capacity of the train")
            .default(trains[&line].capacity.to_string())
            .build(),
        Question::input("origin")
            .message("Enter the new origin of the train")
            .default(trains[&line].origin.clone())
            .build(),
        Question::input("destination")
            .message("Enter the new destination of the train")
            .default(trains[&line].destination.clone())
            .build(),
        Question::input("departure")
            .message("Enter the new departure time of the train (HH:MM)")
            .default(trains[&line].departure.format("%H:%M").to_string())
            .build(),
        Question::input("arrival")
            .message("Enter the new arrival time of the train (HH:MM)")
            .default(trains[&line].arrival.format("%H:%M").to_string())
            .build(),
    ];

    let answers = requestty::prompt(questions)?;

    if let Some(train) = trains.get_mut(&line) {
        train.name = answers.get("name").unwrap().as_string().unwrap().to_string();
        train.capacity = answers.get("capacity").unwrap().as_string().unwrap().parse().unwrap();
        train.origin = answers.get("origin").unwrap().as_string().unwrap().to_string();
        train.destination = answers.get("destination").unwrap().as_string().unwrap().to_string();
        train.departure = NaiveTime::parse_from_str(answers.get("departure").unwrap().as_string().unwrap(), "%H:%M")
            .context("Invalid departure time")?;
        train.arrival = NaiveTime::parse_from_str(answers.get("arrival").unwrap().as_string().unwrap(), "%H:%M")
            .context("Invalid arrival time")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveTime;

    #[test]
    fn test_manage_trains_add() {
        let mut trains = TrainList::new();
        let train = Train::new(
            1,
            "Express".to_string(),
            100,
            "City A".to_string(),
            "City B".to_string(),
            NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
        );

        trains.insert(train.line, train);

        assert_eq!(trains.len(), 1);
        assert!(trains.contains_key(&1));
    }

    #[test]
    fn test_manage_trains_remove() {
        let mut trains = TrainList::new();
        let train = Train::new(
            1,
            "Express".to_string(),
            100,
            "City A".to_string(),
            "City B".to_string(),
            NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
        );

        trains.insert(train.line, train);
        trains.remove(&1);

        assert_eq!(trains.len(), 0);
        assert!(!trains.contains_key(&1));
    }

    #[test]
    fn test_manage_trains_list() {
        let mut trains = TrainList::new();
        let train = Train::new(
            1,
            "Express".to_string(),
            100,
            "City A".to_string(),
            "City B".to_string(),
            NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
        );

        trains.insert(train.line, train);

        assert_eq!(trains.len(), 1);
        assert!(trains.contains_key(&1));
    }

    #[test]
    fn test_edit_train() {
        let mut trains = TrainList::new();
        let train = Train::new(
            1,
            "Express".to_string(),
            100,
            "City A".to_string(),
            "City B".to_string(),
            NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
        );

        trains.insert(train.line, train);

        // Edit the train details directly
        if let Some(train) = trains.get_mut(&1) {
            train.name = "Super Express".to_string();
            train.capacity = 200;
            train.origin = "City X".to_string();
            train.destination = "City Y".to_string();
            train.departure = NaiveTime::from_hms_opt(10, 0, 0).unwrap();
            train.arrival = NaiveTime::from_hms_opt(13, 0, 0).unwrap();
        }

        let edited_train = trains.get(&1).unwrap();
        assert_eq!(edited_train.name, "Super Express");
        assert_eq!(edited_train.capacity, 200);
        assert_eq!(edited_train.origin, "City X");
        assert_eq!(edited_train.destination, "City Y");
        assert_eq!(edited_train.departure, NaiveTime::from_hms_opt(10, 0, 0).unwrap());
        assert_eq!(edited_train.arrival, NaiveTime::from_hms_opt(13, 0, 0).unwrap());
    }
}
