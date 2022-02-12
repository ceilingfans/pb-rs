extern crate termion;

use rustyline::{Editor, error::ReadlineError};
use termion::{color, style};

use crate::util::formatter;

fn read_u8(prompt: &str, typ: &str, max: u8) -> Result<u8, &'static str> {
    println!(
        formatter::format_prompt(prompt)
    );
    let mut rl = Editor::<()>::new();
    let ret: u8;
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                let n = line.parse::<u8>();
                if let Ok(num) = n {
                    if n > max {
                        println!(formatter::format_error(format!("maximum of {} for {}", max, typ).as_str()));
                        continue;
                    }
                    ret = num;
                    break;
                } else {
                    println!(formatter::format_error("number entered was invalid"));
                    continue;
                }
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                println!("Successfully exited.");
                std::process::exit(0);
            },
            Err(e) => {
                println!("Unknown Error: {}", e);
                std::process::exit(1);
            }
        }
    }
    Ok(ret)
}

pub fn read_players() -> Result<u8, &'static str> {
    read_u8("Enter the number of players", "players", 6)
}

pub fn read_substr_len() -> Result<u8, &'static str> {
    read_u8("Enter the substring length", "substring", 10)
}
