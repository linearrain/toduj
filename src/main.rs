use colored::*;
use std::io::Write;

pub mod languages;
use languages::Langs;

pub mod todo_logic;
use todo_logic::*;

pub mod io_out;
use io_out::*;

pub mod help;
use help::help_message;

pub mod task_manager;

pub struct App {
    langs_available : Vec<(String, String)>,
    lang_code : usize,
}

pub enum Help {
    SeeTasks, UnknownCommand
}

fn language_select(app_settings : &mut App, user_lang : &String) -> usize {
    let mut counter : usize = 0;
    for lang in &app_settings.langs_available {
        if *user_lang == *lang.0 {
            app_settings.lang_code = counter;
            return counter;
        }
        counter += 1;
    }

    counter
}

fn main() {
    let mut app_settings = App {
        langs_available: vec![("sk".to_string(), "Slovenčina".to_string()),
        ("ua".to_string(), "Українська".to_string()), ("en".to_string(), "English".to_string())],
        lang_code: 0,
    };

    clearscreen::clear().unwrap();

    app_print("Welcome! To start, select the preffered language:", false);

    for lang in &app_settings.langs_available {
        println!("{}\t({})", lang.1.blue(), lang.0.green());
    }

    let user_lang = get_input();

    let mut lang = language_select(&mut app_settings, &user_lang);

    while lang == app_settings.langs_available.len() {
        app_print("There was a trouble during the language selection. No following language code is provided. Type the languge code you want to have the app set in. For example: ", true);
        print!("{}", &app_settings.langs_available.get(1).unwrap().0.blue());
        std::io::stdout().flush().unwrap();
        let user_lang = get_input();
        lang = language_select(&mut app_settings, &user_lang);
    }

    println!("{} is selected. Further services will be provided in the following language: {}", &app_settings.lang_code.to_string().blue(), &app_settings.langs_available.get(lang).unwrap().1.to_string().blue());

    let langs : Langs = io_out::set_langs();

    todo_logic::start_app(&app_settings, &langs);
}
