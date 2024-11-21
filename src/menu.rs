use anyhow::Result;
use requestty::{self, Question};

pub enum Menu {
    Trains,
    Passengers,
    Bookings,
    Exit,
}

pub fn main_menu_prompt() -> Result<Menu> {
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
