use crate::{Help, Langs, App};
use colored::*;

// A small module for calling the help message in case user confused the command or needs
// assistance

pub fn help_message(issue : Help, app_sets : &App, languages : &Langs) {
    println!("{} {}", "[Toduj Help]".blue(), 

        match issue {
            Help::SeeTasks        => languages.lang(app_sets.lang_code).tasks_commands.as_str(),
            Help::UnknownCommand  => languages.lang(app_sets.lang_code).tasks_commands.as_str(),
        }
    );
    println!("             Use {} to return to the menu", "back".blue())
}
