use requestty::{prompt_one, Answers, Question};
use std::num::ParseIntError;
use std::process::Command;

pub fn clear_screen() {
    Command::new("clear").status().unwrap();
}

pub fn continue_prompt() {
    prompt_one(Question::input("continue").message("Press Enter to continue").build()).unwrap();
}

pub fn parse_string_answer(
    answers: &Answers,
    field: &str,
    delimiter: &str,
) -> String {
    answers
        .get(field)
        .unwrap()
        .as_list_item()
        .unwrap()
        .text
        .split(delimiter)
        .next()
        .unwrap()
        .trim()
        .to_string()
}

pub fn parse_number_answer(
    answers: Answers,
    field: &str,
    delimiter: &str,
) -> Result<u32, ParseIntError> {
    answers
        .get(field)
        .unwrap()
        .as_list_item()
        .unwrap()
        .text
        .split(delimiter)
        .next()
        .unwrap()
        .trim()
        .parse::<u32>()
}
