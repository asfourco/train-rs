use trains_rs::booking_system::{
    add_booking, edit_booking, remove_booking, Booking, BookingList, Passenger, PassengerList,
    Train, TrainList,
};
use chrono::NaiveTime;

#[test]
fn test_add_booking() {
    let mut passengers = PassengerList::new();
    let mut trains = TrainList::new();
    let mut bookings = BookingList::new();

    let passenger = Passenger::new("1".to_string(), "John Doe".to_string(), 30);
    passengers.insert(passenger.id.clone(), passenger);

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

    let result = add_booking(
        &mut bookings,
        &mut passengers,
        &mut trains,
        "1".to_string(),
        1,
    );
    assert!(result.is_ok());

    let booking_id = "1_1".to_string();
    assert!(bookings.contains_key(&booking_id));
    assert!(passengers.get("1").unwrap().bookings.contains(&booking_id));
    assert!(trains.get(&1).unwrap().passengers.contains("1"));
}

#[test]
fn test_edit_booking() {
    let mut passengers = PassengerList::new();
    let mut trains = TrainList::new();
    let mut bookings = BookingList::new();

    let passenger = Passenger::new("1".to_string(), "John Doe".to_string(), 30);
    passengers.insert(passenger.id.clone(), passenger);

    let train1 = Train::new(
        1,
        "Express".to_string(),
        100,
        "City A".to_string(),
        "City B".to_string(),
        NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
        NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
    );
    trains.insert(train1.line, train1);

    let train2 = Train::new(
        2,
        "Super Express".to_string(),
        100,
        "City C".to_string(),
        "City D".to_string(),
        NaiveTime::from_hms_opt(13, 0, 0).unwrap(),
        NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
    );
    trains.insert(train2.line, train2);

    add_booking(
        &mut bookings,
        &mut passengers,
        &mut trains,
        "1".to_string(),
        1,
    )
    .unwrap();

    let result = edit_booking(&mut bookings, &mut passengers, &mut trains, "1_1", 2);
    assert!(result.is_ok());

    let booking = bookings.get("1_1").unwrap();
    assert_eq!(booking.train_line, 2);
}

#[test]
fn test_remove_booking() {
    let mut passengers = PassengerList::new();
    let mut trains = TrainList::new();
    let mut bookings = BookingList::new();

    let passenger = Passenger::new("1".to_string(), "John Doe".to_string(), 30);
    passengers.insert(passenger.id.clone(), passenger);

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

    add_booking(
        &mut bookings,
        &mut passengers,
        &mut trains,
        "1".to_string(),
        1,
    )
    .unwrap();

    let result = remove_booking(
        &mut bookings,
        &mut passengers,
        &mut trains,
        "1_1".to_string(),
    );
    assert!(result.is_ok());

    let booking_id = "1_1".to_string();
    assert!(!bookings.contains_key(&booking_id));
    assert!(!passengers.get("1").unwrap().bookings.contains(&booking_id));
    assert!(!trains.get(&1).unwrap().passengers.contains("1"));
}
