# Train Ticketing System

Welcome to the Train Ticketing System, a Rust-based application for managing train bookings, passengers, and trains.

## Features

- Manage trains: Add, edit, and remove trains.
- Manage passengers: Add, edit, and remove passengers.
- Manage bookings: Add, edit, and remove bookings.
- List all bookings, bookings for a specific passenger, and passengers on a specific train.

## Dependencies

- `anyhow`: Error handling library.
- `chrono`: Date and time library.
- `requestty`: Library for interactive prompts.

## Installation

1. Ensure you have Rust installed. If not, you can install it from [here](https://www.rust-lang.org/tools/install).
2. Clone the repository:
    ```bash
    git clone https://github.com/asfourco/train-rs.git
    cd train-rs
    ```
3. Build the project:
    ```bash
    make build
    ```

## Usage

1. Run the application:
    ```bash
    make run
    ```
2. Follow the on-screen prompts to manage trains, passengers, and bookings.

## Project Structure

- `main.rs`: Entry point of the application.
- `actions.rs`: Contains actions for adding and removing items.
- `bookings.rs`: Manages booking-related functionality.
- `helper.rs`: Helper functions.
- `menu.rs`: Menu-related functionality.
- `passengers.rs`: Manages passenger-related functionality.
- `trains.rs`: Manages train-related functionality.

## Example

Upon running the application, you will be greeted with a welcome message and a menu to choose from:

```
Welcome to the Train Ticketing system. Please choose an option from the menu below:

> Trains
> Passengers
> Bookings
> Exit
```

Choose an option by entering the corresponding number and follow the prompts to perform the desired actions.

## Testing

To run the tests, use the following command:

```bash
make test
```

License
This project is licensed under the MIT License. See the LICENSE file for details.

