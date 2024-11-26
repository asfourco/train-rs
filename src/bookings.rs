use anyhow::{Context, Result};
use requestty::{self, Question};
use std::collections::HashMap;

use crate::actions::{add_remove_prompt, Action};
use crate::helper::{clear_screen, continue_prompt};
use crate::passengers::PassengerList;
use crate::trains::TrainList;

pub struct Booking {
    id: String,
    passenger_id: String,
    train_line: u32,
}

impl Booking {
    fn new(id: String, passenger_id: String, train_line: u32) -> Self {
        Self {
            id,
            passenger_id,
            train_line,
        }
    }
}

enum BookingListType {
    All,
    Passenger,
    Train,
}

pub type BookingList = HashMap<String, Booking>;

pub fn manage_bookings(
    bookings: &mut BookingList,
    passengers: &mut PassengerList,
    trains: &mut TrainList,
) -> Result<()> {
    loop {
        clear_screen();
        match add_remove_prompt()? {
            Action::List => {
                let list_type = list_bookings()?;
                manage_list_bookings(bookings, passengers, trains, list_type)?;
                continue_prompt();
            }
            Action::Add => {
                let passenger_choices: Vec<String> = passengers
                    .values()
                    .map(|passenger| format!("{}, {}", passenger.id, passenger.name))
                    .collect();
                let train_choices: Vec<String> = trains
                    .values()
                    .map(|train| format!("{}, {}", train.line, train.name))
                    .collect();

                let questions: Vec<Question> = vec![
                    Question::select("passenger")
                        .message("Select passenger")
                        .choices(passenger_choices)
                        .build(),
                    Question::select("train")
                        .message("Select train")
                        .choices(train_choices)
                        .build(),
                ];

                let answers = requestty::prompt(questions)?;

                let passenger_id = answers
                    .get("passenger")
                    .unwrap()
                    .as_list_item()
                    .unwrap()
                    .text
                    .split(',')
                    .next()
                    .unwrap()
                    .trim()
                    .to_string();
                let train_line = answers
                    .get("train")
                    .unwrap()
                    .as_list_item()
                    .unwrap()
                    .text
                    .split(',')
                    .next()
                    .unwrap()
                    .trim()
                    .parse::<u32>()
                    .unwrap();
                add_booking(bookings, passengers, trains, passenger_id, train_line)?;
            }
            Action::Remove => {
                if bookings.is_empty() {
                    println!("No bookings to remove");
                    continue_prompt();
                } else {
                    let booking_choices: Vec<String> = bookings
                        .values()
                        .map(|booking| format!("{}, {}", booking.id, booking.train_line))
                        .collect();
                    let question = Question::select("booking_list")
                        .message("Select booking to delete")
                        .choices(booking_choices)
                        .build();
                    let selection = requestty::prompt_one(question)?;
                    let selected_booking = selection.as_list_item().unwrap().text.clone();
                    remove_booking(bookings, passengers, trains, selected_booking)?;
                }
            }
            Action::Edit => {
                if bookings.is_empty() {
                    println!("No bookings to edit");
                    continue_prompt();
                } else {
                    let booking_choices: Vec<String> = bookings
                        .values()
                        .map(|booking| format!("{}, {}", booking.id, booking.train_line))
                        .collect();
                    let question = Question::select("booking_list")
                        .message("Select booking to edit")
                        .choices(booking_choices)
                        .build();
                    let selection = requestty::prompt_one(question)?;
                    let selected_booking = selection.as_list_item().unwrap().text.clone();
                    let booking_id = selected_booking
                        .split(',')
                        .next()
                        .unwrap()
                        .trim()
                        .to_string();

                    let questions: Vec<Question> = vec![Question::select("train")
                        .message("Select new train")
                        .choices(
                            trains
                                .values()
                                .map(|train| {
                                    requestty::Choice(format!("{}, {}", train.line, train.name))
                                })
                                .collect::<Vec<_>>(),
                        )
                        .build()];

                    let answers = requestty::prompt(questions)?;

                    let train_line = answers
                        .get("train")
                        .unwrap()
                        .as_list_item()
                        .unwrap()
                        .text
                        .split(',')
                        .next()
                        .unwrap()
                        .trim()
                        .parse::<u32>()
                        .unwrap();
                    edit_booking(bookings, passengers, trains, booking_id, train_line)?;
                }
            }
            Action::Back => {
                break;
            }
        }
    }

    Ok(())
}

fn list_bookings() -> Result<BookingListType> {
    let questions = vec![Question::select("list_type")
        .message("Select list type")
        .choice("All bookings")
        .choice("Bookings for passenger")
        .choice("Passengers on train")
        .build()];

    let answers = requestty::prompt(questions)?;

    match answers
        .get("list_type")
        .unwrap()
        .as_list_item()
        .unwrap()
        .index
    {
        0 => Ok(BookingListType::All),
        1 => Ok(BookingListType::Passenger),
        2 => Ok(BookingListType::Train),
        _ => unreachable!(),
    }
}

fn manage_list_bookings(
    bookings: &BookingList,
    passengers: &PassengerList,
    trains: &TrainList,
    list_type: BookingListType,
) -> Result<()> {
    match list_type {
        BookingListType::All => {
            list_all_bookings(bookings)?;
        }
        BookingListType::Passenger => {
            let passenger_id_prompt = requestty::prompt_one(
                Question::input("passenger_id").message("Enter passenger ID"),
            )?;
            let passenger_id = passenger_id_prompt.as_string().unwrap();

            list_bookings_for_passenger(passengers, passenger_id)?;
        }
        BookingListType::Train => {
            let train_line =
                requestty::prompt_one(Question::input("train_line").message("Enter train line"))?
                    .as_string()
                    .unwrap()
                    .parse::<u32>()
                    .context("Invalid train line")?;

            list_passengers_on_train(trains, train_line)?;
        }
    }

    Ok(())
}

pub fn list_all_bookings(bookings: &BookingList) -> Result<()> {
    if bookings.is_empty() {
        println!("No bookings found");
    } else {
        for (id, booking) in bookings {
            println!(
                "ID: {}, Passenger ID: {}, Train Line: {}",
                id, booking.passenger_id, booking.train_line
            );
        }
    }
    Ok(())
}

pub fn add_booking(
    bookings: &mut BookingList,
    passengers: &mut PassengerList,
    trains: &mut TrainList,
    passenger_id: String,
    train_line: u32,
) -> Result<()> {
    let passenger = passengers
        .get(&passenger_id)
        .ok_or_else(|| anyhow::anyhow!("Passenger not found"))?;

    let train = trains
        .get(&train_line)
        .ok_or_else(|| anyhow::anyhow!("Train not found"))?;

    for booking_id in &passenger.bookings {
        let booking = bookings.get(booking_id).unwrap();
        let booked_train = trains.get(&booking.train_line).unwrap();
        if booking.train_line == train_line
            || (train.departure < booked_train.arrival && train.arrival > booked_train.departure)
        {
            return Err(anyhow::anyhow!(
                "Passenger already has a booking for this train or overlapping travel times"
            ));
        }
    }

    let booking_id = format!("{}_{}", passenger_id, train_line);
    let booking = Booking::new(booking_id.clone(), passenger_id.clone(), train_line);

    bookings.insert(booking_id.clone(), booking);
    passengers
        .get_mut(&passenger_id)
        .unwrap()
        .add_booking(booking_id.clone());
    trains
        .get_mut(&train_line)
        .unwrap()
        .passengers
        .insert(passenger_id.to_string());

    Ok(())
}

pub fn edit_booking(
    bookings: &mut BookingList,
    passengers: &mut PassengerList,
    trains: &mut TrainList,
    booking_id: String,
    train_line: u32,
) -> Result<()> {
    let booking = bookings.get(&booking_id).unwrap();
    let passenger = passengers.get(&booking.passenger_id).unwrap();
    let train = trains.get(&train_line).unwrap();

    // Check for overlapping travel times
    for existing_booking_id in &passenger.bookings {
        let existing_booking = bookings.get(existing_booking_id).unwrap();
        let existing_train = trains.get(&existing_booking.train_line).unwrap();
        if train_line == existing_booking.train_line
            || (train.departure < existing_train.arrival
                && train.arrival > existing_train.departure)
        {
            return Err(anyhow::anyhow!(
                "Passenger already has a booking for this train or overlapping travel times"
            ));
        }
    }

    // Update booking
    bookings.get_mut(&booking_id).unwrap().train_line = train_line;

    Ok(())
}

pub fn remove_booking(
    bookings: &mut BookingList,
    passengers: &mut PassengerList,
    trains: &mut TrainList,
    selected_booking: String,
) -> Result<()> {
    let booking_id = selected_booking
        .split(',')
        .next()
        .unwrap()
        .trim()
        .to_string();

    let booking = bookings.remove(&booking_id).unwrap();
    passengers
        .get_mut(&booking.passenger_id)
        .unwrap()
        .remove_booking(&booking_id);
    trains
        .get_mut(&booking.train_line)
        .unwrap()
        .passengers
        .remove(&booking.passenger_id);

    Ok(())
}

pub fn list_passengers_on_train(trains: &TrainList, train_line: u32) -> Result<()> {
    if let Some(train) = trains.get(&train_line) {
        if train.passengers.is_empty() {
            println!("No passengers found on this train");
        } else {
            for passenger_id in &train.passengers {
                println!("Passenger ID: {}", passenger_id);
            }
        }
    } else {
        println!("Train not found");
    }

    Ok(())
}

pub fn list_bookings_for_passenger(passengers: &PassengerList, passenger_id: &str) -> Result<()> {
    if let Some(passenger) = passengers.get(passenger_id) {
        if passenger.bookings.is_empty() {
            println!("No bookings found for this passenger");
        } else {
            for booking_id in &passenger.bookings {
                println!("Booking ID: {}", booking_id);
            }
        }
    } else {
        println!("Passenger not found");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::passengers::Passenger;
    use crate::trains::Train;
    use chrono::NaiveTime;
    use std::collections::HashSet;

    fn setup() -> (BookingList, PassengerList, TrainList) {
        let bookings = BookingList::new();
        let mut passengers = PassengerList::new();
        let mut trains = TrainList::new();

        passengers.insert(
            "P1".to_string(),
            Passenger {
                id: "P1".to_string(),
                name: "John Doe".to_string(),
                age: 30,
                bookings: HashSet::new(),
            },
        );

        trains.insert(
            1,
            Train {
                line: 1,
                name: "Express".to_string(),
                capacity: 100,
                origin: "Toronto".to_string(),
                destination: "Hamilton".to_string(),
                departure: NaiveTime::parse_from_str("11:00", "%H:%M").unwrap(),
                arrival: NaiveTime::parse_from_str("12:00", "%H:%M").unwrap(),
                passengers: HashSet::new(),
            },
        );

        (bookings, passengers, trains)
    }

    #[test]
    fn test_add_booking() {
        let (mut bookings, mut passengers, mut trains) = setup();
        let result = add_booking(
            &mut bookings,
            &mut passengers,
            &mut trains,
            "P1".to_string(),
            1,
        );
        assert!(result.is_ok());
        assert_eq!(bookings.len(), 1);
        assert_eq!(passengers.get("P1").unwrap().bookings.len(), 1);
        assert_eq!(trains.get(&1).unwrap().passengers.len(), 1);
    }

    #[test]
    fn test_remove_booking() {
        let (mut bookings, mut passengers, mut trains) = setup();
        add_booking(
            &mut bookings,
            &mut passengers,
            &mut trains,
            "P1".to_string(),
            1,
        )
        .unwrap();
        let result = remove_booking(
            &mut bookings,
            &mut passengers,
            &mut trains,
            "P1_1".to_string(),
        );
        assert!(result.is_ok());
        assert!(bookings.is_empty());
        assert!(passengers.get("P1").unwrap().bookings.is_empty());
        assert!(trains.get(&1).unwrap().passengers.is_empty());
    }

    #[test]
    fn test_list_all_bookings() {
        let (mut bookings, mut passengers, mut trains) = setup();
        add_booking(
            &mut bookings,
            &mut passengers,
            &mut trains,
            "P1".to_string(),
            1,
        )
        .unwrap();
        let result = list_all_bookings(&bookings);
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_bookings_for_passenger() {
        let (mut bookings, mut passengers, mut trains) = setup();
        add_booking(
            &mut bookings,
            &mut passengers,
            &mut trains,
            "P1".to_string(),
            1,
        )
        .unwrap();
        let result = list_bookings_for_passenger(&passengers, "P1");
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_passengers_on_train() {
        let (mut bookings, mut passengers, mut trains) = setup();
        add_booking(
            &mut bookings,
            &mut passengers,
            &mut trains,
            "P1".to_string(),
            1,
        )
        .unwrap();
        let result = list_passengers_on_train(&trains, 1);
        assert!(result.is_ok());
    }

    #[test]
    fn test_edit_booking() {
        let (mut bookings, mut passengers, mut trains) = setup();

        // Setup initial booking
        add_booking(
            &mut bookings,
            &mut passengers,
            &mut trains,
            "P1".to_string(),
            1,
        )
        .unwrap();

        // Add second train
        trains.insert(
            2,
            Train {
                line: 2,
                name: "Local".to_string(),
                capacity: 100,
                origin: "Toronto".to_string(),
                destination: "Hamilton".to_string(),
                departure: NaiveTime::parse_from_str("13:00", "%H:%M").unwrap(),
                arrival: NaiveTime::parse_from_str("15:00", "%H:%M").unwrap(),
                passengers: HashSet::new(),
            },
        );

        // Test core edit booking function
        let result = edit_booking(
            &mut bookings,
            &mut passengers,
            &mut trains,
            "P1_1".to_string(),
            2,
        );

        assert!(result.is_ok());
        assert_eq!(bookings.get("P1_1").unwrap().train_line, 2);
    }
}
