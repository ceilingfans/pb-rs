extern crate termion;

use std::collections::HashSet;

use rustyline::{error::ReadlineError, Editor};
use termion::clear;

use crate::util::formatter;

fn read_u8(prompt: &str, typ: &str, max: u8, min: u8) -> Result<u8, &'static str> {
    println!("{}{}", clear::All, formatter::format_prompt(prompt));
    let mut rl = Editor::<()>::new();
    let ret: u8;
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                let n = line.parse::<u8>();
                if let Ok(num) = n {
                    if num > max {
                        println!(
                            "{}",
                            formatter::format_error(
                                format!("maximum of {} for {}", max, typ).as_str()
                            )
                        );
                        continue;
                    }
                    if num < min {
                        println!(
                            "{}",
                            formatter::format_error(
                                format!("minimum of {} for {}", min, typ).as_str()
                            )
                        );
                        continue;
                    }
                    ret = num;
                    break;
                } else {
                    println!("{}", formatter::format_error("number entered was invalid"));
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
    println!(
        "{}{}",
        clear::All,
        formatter::format_prompt("Do you want assistance? (y)es/(n)o")
    );
    let mut rl = Editor::<()>::new();
    let mut choices = vec!["n", "no"];
    let mut yes = vec!["y", "ye", "yes"];
    choices.append(&mut yes);
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                let lower = line.to_lowercase();
                if !choices.contains(&lower.as_str()) {
                    println!("{}", formatter::format_error("invalid choice"));
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
    println!(
        "{}{}",
        clear::All,
        formatter::format_prompt("Enter the characters you want to play with\nType '-exit' to stop\n     '-chars' to list all inputs")
    );
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
                        println!(
                            "{}",
                            formatter::format_error("cannot exit before at least 2 characters")
                        );
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
