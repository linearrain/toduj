use crate::{App, Help, io_out::*, task_manager::*, languages::Langs, help::help_message};
use colored::*;
use std::io::Write;

fn add_task(app_sets : &App, languages : &Langs) {
    clearscreen::clear().unwrap();
    println!("{} {}", languages.lang(app_sets.lang_code).task_creator.green(), languages.lang(app_sets.lang_code).task_creator_intro);
    print!("{} >> ", languages.lang(app_sets.lang_code).name.blue());
    std::io::stdout().flush().unwrap();

    let name     : String = get_input();
    let mut yyyy : u16;
    let mut mm   : u16;
    let mut dd   : u16;

    let handle_number_part = |vec : &Vec<&str>, i : usize| {
        match vec[i].parse::<u16>() {
            Ok(number) => number,
            Err(_)     => 0
        }
    };

    loop {
        print!("{} 2045-03-01 >> ", languages.lang(app_sets.lang_code).date.blue());
        std::io::stdout().flush().unwrap();
        let date : String = get_input();
        let date : Vec<&str> = date.split("-").collect();


        if date.len() < 3 {
            app_print("Invalid Date Format", true, true);
            continue;
        }

        yyyy = handle_number_part(&date, 0);
        mm   = handle_number_part(&date, 1);
        dd   = handle_number_part(&date, 2);

        if (1970 > yyyy) || (mm > 12 || mm < 1) || (dd < 1 || dd > 31) {
            app_print("Invalid Date Format", true, true);
            continue;
        }

        println!("{}/{}/{}", yyyy, mm, dd);
        break;
    }

    let mut hrs : u16 = 0;
    let mut mnt : u16 = 1;

    loop {
        print!("{} 00:01 >> ", languages.lang(app_sets.lang_code).time.blue());
        std::io::stdout().flush().unwrap();
        let time = get_input();

        if time.is_empty() {
            app_print("Wrong time format", true, true);
            continue;
        }
        let time : Vec<&str> = time.split(':').collect();

        if time.len() < 2 {
            app_print("Wrong time format", true, true);
            continue;
        }

        hrs = handle_number_part(&time, 0);
        mnt = handle_number_part(&time, 1);

        if hrs < 24 && mnt > 0 && mnt < 60 {
            break;
        }   
    }

    let task = Task {name, yyyy, mm, dd, hrs, mnt, crossed : false};
    match task.write() {
        Ok(_)  => app_print("", false, false),
        Err(_) => app_print("Random error occured. Try again.", true, true)
    }

    
}

fn check_vec_on_digits(vec : &Vec<&str>) -> bool {
    for arg in vec {
        if !is_number(arg) || (arg.parse::<i32>().unwrap() < 1) {
            return false
        }
    }

    true
}

pub fn see_tasks(app_sets : &App, languages : &Langs) {
    clearscreen::clear().unwrap();

    let format_times = |fst : u16| {
        if    fst < 10 {"0".to_owned() + &fst.to_string()}
        else {fst.to_string()}
    };

    let tasks = get_data();

    if tasks.is_err() {
        app_print("Your task list is empty", false, true);
        return ;
    }

    let mut tasks = get_data().unwrap();

    let mut counter = 1;

    println!("{:<6}{:<34}{:<16}{:<8}{:<20}", "Num".green().bold().on_white(), "Name".black().bold().on_white(), "Deadline".black().bold().on_white(), "Until".black().bold().on_white(), "Operations".black().bold().on_white());

    tasks.sort_by_key(|task| ((task.yyyy as u64) * 100000000 + (task.mm as u64) * 1000000 
                                                             + (task.dd as u64)) * 10000 
                                                             + (task.hrs as u64) * 100 + (task.mnt as u64));

    for task in &tasks {
        println!("{counter:<5} {:<30}\t{}/{}/{}\t{}:{}\t{}el   {}ark", 
                    match task.crossed {
                        true  => task.name.strikethrough(),
                        false => task.name.white(),
                    }, 
                    task.yyyy, 
                    format_times(task.mm), 
                    format_times(task.dd), 
                    format_times(task.hrs), 
                    format_times(task.mnt),
                    "D".red(), "M".green()
        );
        counter += 1;
    }

    let task_edit = |mut cmd : Vec<&str>| {
        let first_letter = cmd[0];

        let _ = &cmd.swap_remove(0);
        if !check_vec_on_digits(&cmd) {
            app_print("Non-valid digit code. Try again.", true, true);
            return ;
        }

        for num in &cmd {
            if let Some(number) = num.parse::<usize>().ok() {
                if tasks.len() < number || number == 0 {
                    app_print("Non-valid digit code. Try again.", true, true);
                    return ;
                }

                match first_letter {
                    "D" => match delete_task(&tasks[number - 1].name) {
                        Ok(_)  => app_print("You have deleted the task(s).", false, true),
                        Err(_) => app_print("", true, true)
                    },
                    "M" => match mark_task(number - 1) {
                        Ok(_)  => app_print("You have completed the task(s). Congrats!", false, true),
                        Err(_) => app_print("", true, true)
                    },
                     _  => help_message(Help::UnknownCommand, app_sets, languages),
                }
            }
        }

        if cmd.len() < 1 {
            app_print("Wrong Task Command. Use 'help' to see all the available", true, true)
        }
    };

    // Handling the commands in (1) section
    loop {
        let cmd : String = get_input();
        let mut cmd : Vec<&str> = cmd.split(' ').collect();

        match cmd[0] {
            "help" => help_message(Help::SeeTasks, app_sets, languages),
            "back" => break,
            "add"  => { add_task(app_sets, languages); break },
             _     => { 
                 task_edit(cmd);
                 continue;
             },
        }
    }
}

pub fn start_app(app_sets : &App, languages : &Langs) {
    loop {
        clearscreen::clear().unwrap();

        app_print(languages.lang(app_sets.lang_code).create_task.as_str(), false, true);
        let input : String = get_input();

        match input.as_str() {
            "1" | "one" | "jedna" | "один" => see_tasks(app_sets, languages),
            "2" | "two" | "dva"   | "два"  => add_task(app_sets, languages),
            "exit" | "вийти" | "vystup"    => break,
             _ => app_print("Unknown command", true, true)
        }
    }
}
