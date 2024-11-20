use anyhow::{Context, Result};
use chrono::NaiveTime;
use requestty::{self, Question};
use std::process::Command;

enum Menu {
    Trains,
    Passengers,
    Bookings,
    Exit,
}

enum Action {
    List,
    Add,
    Remove,
    Back,
}

struct Train {
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

struct Passenger {
    id: String,
    name: String,
    age: u16,
}

impl Passenger {
    fn new(id: String, name: String, age: u16) -> Self {
        Self { id, name, age }
    }
}

fn clear_screen() {
    Command::new("clear").status().unwrap();
}

fn main_menu_prompt() -> Result<Menu> {
    let selection = requestty::prompt_one(
        Question::select("menu")
            .message("Select a menu")
            .choice("Trains")
            .choice("Passengers")
            .choice("Bookings")
            .choice("Exit"),
    )?;

    match selection.as_list_item().unwrap().index {
        0 => Ok(Menu::Trains),
        1 => Ok(Menu::Passengers),
        2 => Ok(Menu::Bookings),
        3 => Ok(Menu::Exit),
        _ => unreachable!(),
    }
}

fn add_remove_prompt() -> Result<Action> {
    let selection = requestty::prompt_one(
        Question::select("action")
            .message("Select an action")
            .choice("List")
            .choice("Add")
            .choice("Remove")
            .choice("Back"),
    )?;

    match selection.as_list_item().unwrap().index {
        0 => Ok(Action::List),
        1 => Ok(Action::Add),
        2 => Ok(Action::Remove),
        3 => Ok(Action::Back),
        _ => unreachable!(),
    }
}

fn manage_trains(trains: &mut Vec<Train>) -> Result<()> {
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

fn manage_passengers(passengers: &mut Vec<Passenger>) -> Result<()> {
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
fn main() -> Result<()> {
    let mut trains = vec![];
    let mut passengers = vec![];

    println!("Welcome to the Train Ticketing system. Please choose an option from the menu below");

    loop {
        clear_screen();
        match main_menu_prompt()? {
            Menu::Trains => {
                manage_trains(&mut trains)?;
            }
            Menu::Passengers => {
                manage_passengers(&mut passengers)?;
            }
            Menu::Bookings => {
                println!("You selected Bookings");
            }
            Menu::Exit => {
                println!("Exiting...");
                break;
            }
        }
    }

    Ok(())
}
