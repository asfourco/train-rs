use anyhow::Result;

pub mod actions;
pub mod helper;
pub mod menu;
pub mod passengers;
pub mod trains;

use helper::clear_screen;
use menu::{main_menu_prompt, Menu};
use passengers::manage_passengers;
use trains::manage_trains;

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
