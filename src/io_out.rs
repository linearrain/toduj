use std::io::stdin;
use colored::*;
use std::{fs::File, io::Read};
use crate::Langs;

pub fn get_input() -> String {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Input error");
    input.trim().to_string()
}

pub fn app_print(text : &str, is_e : bool) {
    if !is_e {
        println!("{} {}", "[Toduj v1.0]".green(), text);
    }
    else {
        println!("{} {}", "[Toduj v1.0]".red(), text);
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

pub fn set_langs() -> Langs {
    let mut file  = File::open("translates.json").unwrap();
    let mut texts = String::new();
    file.read_to_string(&mut texts).unwrap();
    serde_json::from_str(&mut texts)
        .expect("Translation file is not found or is wrong.") 
}
