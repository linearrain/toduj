use crate::Help;
use colored::*;

pub fn help_message(issue : Help) {
    println!("{} {}", "[Toduj Help]".blue(), 

        match issue {
            Help::SeeTasks       => "D <number> - delete / complete the task\n             E <number> - edit the task details",
            Help::UnknownCommand => "Unknown Command. Please, refer to 'help' to see all the available commands in this part",
        }
    );
    println!("             Use {} to return to the menu", "back".blue())
}
