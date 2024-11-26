use anyhow::Result;
use requestty::{self, Question};
use std::collections::{HashMap, HashSet};

use crate::actions::{add_remove_prompt, Action};
use crate::helper::{clear_screen, continue_prompt};

pub struct Passenger {
    pub id: String,
    pub name: String,
    pub age: u16,
    pub bookings: HashSet<String>,
}

impl Passenger {
    pub fn new(id: String, name: String, age: u16) -> Self {
        Self {
            id,
            name,
            age,
            bookings: HashSet::new(),
        }
    }
    pub fn add_booking(&mut self, booking_id: String) {
        self.bookings.insert(booking_id);
    }
    pub fn remove_booking(&mut self, booking_id: &str) {
        self.bookings.remove(booking_id);
    }
}

pub type PassengerList = HashMap<String, Passenger>;

pub fn manage_passengers(passengers: &mut PassengerList) -> Result<()> {
    loop {
        clear_screen();
        match add_remove_prompt()? {
            Action::List => {
                if passengers.is_empty() {
                    println!("No passengers found");
                } else {
                    for (id, passenger) in &mut *passengers {
                        println!(
                            "ID: {}, Name: {}, Age: {}",
                            id, passenger.name, passenger.age
                        );
                    }
                }
                continue_prompt();
            }
            Action::Add => {
                let new_passenger = add_passenger()?;
                if !passengers.contains_key(&new_passenger.id) {
                    passengers.insert(new_passenger.id.clone(), new_passenger);
                } else {
                    println!("Passenger already exissts");
                    continue_prompt();
                }
            }
            Action::Remove => {
                if passengers.is_empty() {
                    println!("No passengers to remove");
                    continue_prompt();
                } else {
                    remove_passenger(passengers)?;
                }
            }
            Action::Edit => {
                if passengers.is_empty() {
                    println!("No passengers to edit");
                    continue_prompt();
                } else {
                    edit_passenger(passengers)?;
                }
            }
            Action::Back => {
                return Ok(());
            }
        }
    }
}

fn add_passenger() -> Result<Passenger> {
    let questions: Vec<Question> = vec![
        Question::input("id")
            .message("Enter the ID of the passenger")
            .build(),
        Question::input("name")
            .message("Enter the name of the passenger")
            .build(),
        Question::input("age")
            .message("Enter the age of the passenger")
            .build(),
    ];

    let passenger = requestty::prompt(questions)?;

    Ok(Passenger::new(
        passenger
            .get("id")
            .unwrap()
            .as_string()
            .unwrap()
            .to_string(),
        passenger
            .get("name")
            .unwrap()
            .as_string()
            .unwrap()
            .to_string(),
        passenger.get("age").unwrap().as_string().unwrap().parse()?,
    ))
}

fn remove_passenger(passengers: &mut PassengerList) -> Result<()> {
    let passenger_choices: Vec<String> = passengers
        .values()
        .map(|passenger| format!("{}, {}", passenger.id, passenger.name))
        .collect();
    let question = Question::select("passenger_list")
        .message("Select passenger to delete")
        .choices(passenger_choices)
        .build();
    let selection = requestty::prompt_one(question)?;
    let selected_passenger = selection.as_list_item().unwrap().text.clone();
    let id = selected_passenger
        .split(',')
        .next()
        .unwrap()
        .trim()
        .to_string();
    passengers.remove(&id);

    Ok(())
}

fn edit_passenger(passengers: &mut PassengerList) -> Result<()> {
    let passenger_choices: Vec<String> = passengers
        .values()
        .map(|passenger| format!("{}, {}", passenger.id, passenger.name))
        .collect();
    let question = Question::select("passenger_list")
        .message("Select passenger to edit")
        .choices(passenger_choices)
        .build();
    let selection = requestty::prompt_one(question)?;
    let selected_passenger = selection.as_list_item().unwrap().text.clone();
    let id = selected_passenger
        .split(',')
        .next()
        .unwrap()
        .trim()
        .to_string();

    let questions: Vec<Question> = vec![
        Question::input("name")
            .message("Enter the new name of the passenger")
            .default(passengers[&id].name.clone())
            .build(),
        Question::input("age")
            .message("Enter the new age of the passenger")
            .default(passengers[&id].age.to_string())
            .build(),
    ];

    let answers = requestty::prompt(questions)?;

    if let Some(passenger) = passengers.get_mut(&id) {
        passenger.name = answers
            .get("name")
            .unwrap()
            .as_string()
            .unwrap()
            .to_string();
        passenger.age = answers
            .get("age")
            .unwrap()
            .as_string()
            .unwrap()
            .parse()
            .unwrap();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_passenger() {
        let passenger = Passenger::new("1".to_string(), "John Doe".to_string(), 30);

        assert_eq!(passenger.id, "1");
        assert_eq!(passenger.name, "John Doe");
        assert_eq!(passenger.age, 30);
    }

    #[test]
    fn test_manage_passengers_add() {
        let mut passengers = PassengerList::new();
        let passenger = Passenger::new("1".to_string(), "John Doe".to_string(), 30);

        passengers.insert(passenger.id.clone(), passenger);

        assert_eq!(passengers.len(), 1);
        assert!(passengers.contains_key(&"1".to_string()));
    }

    #[test]
    fn test_manage_passengers_remove() {
        let mut passengers = PassengerList::new();
        let passenger = Passenger::new("1".to_string(), "John Doe".to_string(), 30);

        passengers.insert(passenger.id.clone(), passenger);
        passengers.remove(&"1".to_string());

        assert_eq!(passengers.len(), 0);
        assert!(!passengers.contains_key(&"1".to_string()));
    }

    #[test]
    fn test_manage_passengers_list() {
        let mut passengers = PassengerList::new();
        let passenger = Passenger::new("1".to_string(), "John Doe".to_string(), 30);

        passengers.insert(passenger.id.clone(), passenger);

        assert_eq!(passengers.len(), 1);
        assert!(passengers.contains_key(&"1".to_string()));
    }

    #[test]
    fn test_edit_passenger() {
        let mut passengers = PassengerList::new();
        let passenger = Passenger::new("1".to_string(), "John Doe".to_string(), 30);

        passengers.insert(passenger.id.clone(), passenger);

        // Edit the passenger details directly
        if let Some(passenger) = passengers.get_mut(&"1".to_string()) {
            passenger.name = "Jane Doe".to_string();
            passenger.age = 35;
        }

        let edited_passenger = passengers.get(&"1".to_string()).unwrap();
        assert_eq!(edited_passenger.name, "Jane Doe");
        assert_eq!(edited_passenger.age, 35);
    }
}
