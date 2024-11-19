enum Menu {
    Trains,
    Passengers,
    Bookings,
    Exit
}

enum Action {
    Add,
    Remove
}

fn prompt() -> requestty::Result<Menu> {
    let selection = requestty::prompt_one(
       requestty::Question::select("menu")
       .message("Select a menu")
       .choice("Trains")
       .choice("Passengers")
       .choice("Bookings") 
       .choice("Exit")
    )?;

    match selection.as_list_item().unwrap().index {
        0 => Ok(Menu::Trains),
        1 => Ok(Menu::Passengers),
        2 => Ok(Menu::Bookings),
        3 => Ok(Menu::Exit),
        _ => unreachable!(),
    }
}

fn main() {
    println!("Welcome to the Train Ticketing system. Please choose an option from the menu below");

    loop {
        let menu = prompt().unwrap();

        match menu {
            Menu::Trains => {
                println!("You selected Trains");
            },
            Menu::Passengers => {
                println!("You selected Passengers");
            },
            Menu::Bookings => {
                println!("You selected Bookings");
            },
            Menu::Exit => {
                println!("Exiting...");
                break;
            }
        }
    }
}
