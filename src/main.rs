/*  
 *  All the used imports
 */
use colored::*;
use std::{io::{Error, Write}, time::Duration};

pub mod languages;
use languages::Langs;

pub mod todo_logic;
use todo_logic::*;

pub mod io_out;
use io_out::*;

pub mod help;
use help::help_message;

pub mod task_manager;

// A structure, which holds the app language properties
pub struct App<'a> {
    langs_available : Vec<(&'a str, &'a str)>,
    lang_code : usize,
}

// An enum for different help codes
pub enum Help {
    SeeTasks, UnknownCommand
}

// Language selector function
fn language_select(app_settings : &mut App, user_lang : &str) -> Option<usize> {
    app_settings.langs_available.iter()
        .position(|&lan| lan.0 == user_lang)
}

fn main() {
    let mut app_settings = App {
        langs_available: vec![("sk", "Slovenčina"),
                              ("ua", "Українська"), 
                              ("en", "English"),
                              ("cz", "Czech")],
        lang_code: 0,
    };

    clearscreen::clear().unwrap();

    app_print("Welcome! To start, select the preffered language:", false, true);

    for lang in &app_settings.langs_available {
        println!("{}\t({})", lang.1.blue(), lang.0.green());
    }

    let mut lang_code : Option<usize>;

    loop {
        let user_lang : &str = &get_input();
        lang_code = language_select(&mut app_settings, user_lang);

        match lang_code {
            None       => {     
                              app_print("There was a trouble during the language selection. No following language code is provided. Type the languge code you want to have the app set in. For example: ", true, false);
                              print!("{}", &app_settings.langs_available.get(1).unwrap().0.blue()); 
                              std::io::stdout().flush().unwrap(); 
                          },
            Some(code) => {app_settings.lang_code = code; break},
        }
    }

    let (_lan_abbr, lan_name) = &app_settings.langs_available.get(app_settings.lang_code).unwrap();

    println!("{} is selected. Further services will be provided in the following language: {}", 
                &app_settings.lang_code.to_string().blue(), 
                lan_name.green()
            );
    
    for i in (0..=100).step_by(5) {
            print!("\rLoading {i}%");
            std::thread::sleep(Duration::from_millis(60));
            std::io::stdout().flush().unwrap();
    }

    let langs : Langs = io_out::set_langs();

    todo_logic::start_app(&app_settings, &langs);
}
