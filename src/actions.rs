use anyhow::Result;
use requestty::{self, Question};

pub enum Action {
    List,
    Add,
    Remove,
    Edit,
    Back,
}

pub fn add_remove_prompt() -> Result<Action> {
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
        3 => Ok(Action::Edit),
        4 => Ok(Action::Back),
        _ => unreachable!(),
    }
}
