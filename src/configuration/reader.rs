extern crate termion;

use std::collections::HashSet;

use rustyline::{error::ReadlineError, Editor};
use termion::clear;

use crate::util::printer;

fn read_u8(prompt: &str, typ: &str, max: u8, min: u8) -> Result<u8, &'static str> {
    printer::clear_screen();
    printer::print_prompt(prompt);
    let mut rl = Editor::<()>::new();
    let ret: u8;
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                let n = line.parse::<u8>();
                if let Ok(num) = n {
                    if num > max {
                        printer::print_error(format!("maximum of {} for {}", max, typ).as_str());
                        continue;
                    }
                    if num < min {
                        printer::print_error(format!("minimum of {} for {}", max, typ).as_str());
                        continue;
                    }
                    ret = num;
                    break;
                } else {
                    printer::print_error("number entered is invalid");
                    continue;
                }
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                println!("Successfully exited.");
                std::process::exit(0);
            }
            Err(e) => {
                println!("Unknown Error: {}", e);
                std::process::exit(1);
            }
        }
    }
    Ok(ret)
}

pub fn read_players() -> Result<u8, &'static str> {
    read_u8("Enter the number of players", "players", 6, 1)
}

pub fn read_substr_len() -> Result<u8, &'static str> {
    read_u8("Enter the substring length", "substring", 10, 3)
}

pub fn read_assist() -> Result<bool, &'static str> {
    printer::clear_screen();
    printer::print_prompt("Do you want assistance? (y)es/(n)o");
    let mut rl = Editor::<()>::new();
    let no = vec!["n", "no"];
    let yes = vec!["y", "ye", "yes"];
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                let lower = line.to_lowercase();
                if !no.contains(&lower.as_str()) && !yes.contains(&lower.as_str()) {
                    printer::print_error("invalid choice");
                    continue;
                }
                return Ok(yes.contains(&lower.as_str()));
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                println!("Successfully exited.");
                std::process::exit(0);
            }
            Err(e) => {
                println!("Unknown Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

pub fn read_chars() -> Result<HashSet<char>, &'static str> {
    printer::clear_screen();
    printer::print_prompt("Enter the characters you want to play with\nType '-exit' to stop\n     '-chars' to list all inputs");
    let mut ret = HashSet::<char>::new();
    let mut rl = Editor::<()>::new();
    let mut warning = false;
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                println!("{}", line.len());
                let lower = line.to_lowercase();
                if lower == "-exit" {
                    if ret.len() < 2 {
                        printer::print_error("cannot exit before at least 2 characters");
                        warning = true;
                        continue;
                    }
                    return Ok(ret);
                }
                if lower == "-chars" {
                    if ret.len() == 0 {
                        println!("None");
                        continue;
                    }
                    println!("{:?}", &ret);
                    continue;
                }
                if line.len() > 1 {
                    if !warning {
                        println!("warning: entering more than 1 character at once will add all characters separately");
                        warning = true;
                    }
                }
                for c in line.chars() {
                    if ret.contains(&c) {
                        println!("warning: ignoring {} as it has already been added", &c);
                        continue;
                    }
                    ret.insert(c);
                }
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                println!("Successfully exited.");
                std::process::exit(0);
            }
            Err(e) => {
                println!("Unknown Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

pub fn read_choice(set: &HashSet<char>) -> char {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                if line.len() > 1 {
                    println!("warning: using more than 1 character will result in the first character being chosen");
                }
                let input = line.chars().next(); // get first char
                if let Some(c) = input {
                    if set.contains(&c) {
                        return c;
                    } else {
                        printer::print_error(format!("{} is not a valid choice", c).as_str());
                        continue;
                    }
                }
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                println!("Successfully exited.");
                std::process::exit(0);
            }
            Err(e) => {
                println!("Unknown Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
