use requestty::{prompt_one, Question};
use std::process::Command;

pub fn clear_screen() {
    Command::new("clear").status().unwrap();
}

pub fn continue_prompt() {
    prompt_one(
        Question::input("continue")
            .message("Press Enter to continue")
            .build(),
    )
    .unwrap();
}
