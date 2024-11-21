use std::process::Command;

pub fn clear_screen() {
    Command::new("clear").status().unwrap();
}
