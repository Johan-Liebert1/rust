use std::io::{self, Write};

use crate::constants;

pub fn color_print(color: &'static str, string: String, new_line: bool) {
    let string = format!("{} {} {}", color, string, constants::RESET);

    if new_line {
        println!("{}", string);
    } else {
        print!("{}", string);
        io::stdout().flush().unwrap();
    }
}

pub fn color_bg_print(color: &'static str, string: String, new_line: bool) {
    let string = format!(
        "{}{} {} {}",
        color,
        constants::BLACK,
        string,
        constants::RESET
    );

    if new_line {
        println!("{}", string);
    } else {
        print!("{}", string);
        io::stdout().flush().unwrap();
    }
}

pub fn print_color_guesses(guess: String, word: String) {
    let guess_bytes = guess.as_bytes();
    let word_bytes = word.as_bytes();

    for i in 0..guess.len() {
        if guess_bytes[i] == word_bytes[i] {
            color_bg_print(
                constants::BG_GREEN,
                (guess_bytes[i] as char).to_string(),
                false,
            );
        } else if word_bytes.contains(&guess_bytes[i]) {
            color_bg_print(
                constants::BG_YELLOW,
                (guess_bytes[i] as char).to_string(),
                false,
            );
        } else {
            color_bg_print(
                constants::BG_WHITE,
                (guess_bytes[i] as char).to_string(),
                false,
            );
        }
    }

    println!();
}

pub fn clear_line() {
    // move cursor to the start of the line
    io::stdout().write("\u{001b}[0G".as_bytes()).unwrap();
    io::stdout().flush().unwrap();

    // clear to the end of line
    io::stdout().write("\u{001b}[1K".as_bytes()).unwrap();
    io::stdout().flush().unwrap();
}
