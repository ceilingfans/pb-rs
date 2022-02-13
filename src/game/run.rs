extern crate termion;

use rustyline::{error::ReadlineError, Editor};
use std::collections::HashSet;
use substring::Substring;
use termion::clear;

use crate::configuration::{options::*, reader::*};
use crate::util::printer;

fn get_options() -> Options {
    for arg in std::env::args() {
        let lower = arg.to_lowercase();
        if lower == "--default" || lower == "-d" {
            return Options::default();
        }
    }

    Options {
        players: read_players().unwrap(),
        substr_len: read_substr_len().unwrap(),
        assist: read_assist().unwrap(),
        chars: read_chars().unwrap(),
    }
}

pub struct Game {
    options: Options,
    string: String,
}

impl Game {
    pub fn new() -> Self {
        Game {
            options: get_options(),
            string: String::new(),
        }
    }

    fn get_valid_options(&self) -> HashSet<char> {
        if self.string.len() < self.options.substr_len as usize {
            return self.options.chars.clone();
        }

        let mut ret = HashSet::<char>::new();
        let prefix = self.string.substring(
            self.string.len() - (self.options.substr_len - 1) as usize,
            self.string.len(),
        );

        for ch in &self.options.chars {
            if !self.string.contains(format!("{}{}", &prefix, &ch).as_str()) {
                ret.insert(ch.clone());
            }
        }

        ret
    }

    pub fn run(&mut self) -> i32 {
        // TODO: loss stack
        // TODO: single player
        if self.options.players == 1 {
            println!("Single player against a computer has not been implemented yet");
            return 501; // not implemented
        }
        loop {
            for player in 1..self.options.players + 1 {
                let valid_options = self.get_valid_options();
                // start options check
                if valid_options.is_empty() {
                    // single player or 1v1
                    if self.options.players < 3 {
                        printer::print_loss(player);
                        return 0; // success
                    }
                    // if number of players is more than 2, the player who caused the game to end
                    // through making someone run out of options
                    else if self.options.players > 2 {
                        if player == 1 {
                            printer::print_win(self.options.players);
                        } else {
                            printer::print_win(player - 1);
                        }
                        return 0; // success
                    }
                }
                // end options check

                // assistance mode
                if self.options.assist {
                    println!("Valid Options: {:?}", &valid_options);
                }

                loop {
                    let c = read_choice(&self.options.chars);
                    if !self.options.chars.contains(&c) {
                        printer::print_error(format!("{} is not a valid choice", c).as_str());
                        continue;
                    }
                    if !valid_options.contains(&c) {
                        if self.options.assist {
                            printer::print_error(format!("{} will make you lose!", c).as_str());
                            continue;
                        } else {
                            printer::print_loss(player);
                            return 0; // success
                        }
                    }

                    self.string.push(c);
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_valid_options_default() {
        let g = Game {
            string: String::from("000010002000"),
            options: Options::default(),
        };
        // it shouldn't have any valid options
        assert_eq!(g.get_valid_options().len(), 0);
    }

    #[test]
    fn get_valid_options_all_valid() {
        let g1 = Game {
            string: String::new(),
            options: Options::default(),
        };
        // it should have all options
        assert_eq!(g1.get_valid_options().len(), 3);
    }

    #[test]
    fn get_valid_options_custom_substr_len() {
        let mut opt = Options::default();
        opt.substr_len = 3;
        let g2 = Game {
            string: String::from("000100200"),
            options: opt,
        };
        // it should have no valid options
        assert_eq!(g2.get_valid_options().len(), 0);
    }
}
