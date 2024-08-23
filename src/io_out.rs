use std::io::stdin;
use colored::*;
use std::{fs::File, io::Read};
use crate::Langs;

// Toduj heavily relies on user input, so to shorten the code this function was created 
pub fn get_input() -> String {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Input error");
    input.trim().to_string()
}

// A function, which creates a markup for program messages. Supports both error output and normal
// one 
pub fn app_print(text : &str, is_e : bool, nline : bool) {
    let newline = || -> &str {
        match nline {
            false => "",
            true  => "\n",
        }
    };

    match is_e {
        false => print!("{} {}{}", "[Toduj v1.0]".green(), text, newline()),
        true  => print!("{} {}{}", "[Toduj v1.0]".red(), text, newline()),
    }
}

pub fn is_number(num : &str) -> bool {
    for digit in num.chars() {
        if !digit.is_numeric() {
            return false
        }
    }

    true
}

// A fucntion used to set the language of the program
pub fn set_langs() -> Langs {
    let mut file  = File::open("translates.json").unwrap();
    let mut texts = String::new();
    file.read_to_string(&mut texts).unwrap();
    serde_json::from_str(&mut texts)
        .expect("Translation file is not found or is wrong.") 
}
