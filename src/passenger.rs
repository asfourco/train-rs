use anyhow::Result;
use requestty::{self, Question};

use crate::actions::{add_remove_prompt, Action};
use crate::helper::clear_screen;

pub struct Passenger {
    id: String,
    name: String,
    age: u16,
}

impl Passenger {
    fn new(id: String, name: String, age: u16) -> Self {
        Self { id, name, age }
    }
}

pub fn manage_passengers(passengers: &mut Vec<Passenger>) -> Result<()> {
    loop {
        clear_screen();
        match add_remove_prompt()? {
            Action::List => {
                if passengers.is_empty() {
                    println!("No passengers found");
                }
                for passenger in passengers.iter() {
                    println!(
                        "ID: {}, Name: {}, Age: {}",
                        passenger.id, passenger.name, passenger.age
                    );
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
                passengers.push(add_passenger()?);
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
            .parse()
            .unwrap(),
        passenger
            .get("name")
            .unwrap()
            .as_string()
            .unwrap()
            .to_string(),
        passenger
            .get("age")
            .unwrap()
            .as_string()
            .unwrap()
            .parse()
            .unwrap(),
    ))
}
