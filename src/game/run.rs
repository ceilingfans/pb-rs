use std::collections::HashSet;
use substring::Substring;

use crate::configuration::{options::*, reader::*};
use crate::util::printer;
use crate::util::printer::print_loss_computer;

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

    fn get_options(&self, ch: char) -> u8 {
        let mut choices: u8 = 0;
        let string = format!("{}{}", &self.string, ch);
        let prefix = string.substring(
            string.len() - (self.options.substr_len - 1) as usize,
            string.len(),
        );

        for c in &self.options.chars {
            if !string.contains(format!("{}{}", &prefix, &c).as_str()) {
                choices += 1;
            }
        }

        choices
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
        printer::clear_screen();
        // TODO: single player
        if self.options.players == 1 {
            let start = read_start().unwrap();
            loop {
                for player in 1..3 {
                    let valid_options = self.get_valid_options();
                    // options check
                    if valid_options.is_empty() {
                        printer::print_loss_computer(player, start);
                        return 0; // success
                    }

                    // human
                    if (player == 1 && start) || (player == 2 && !start) {
                        // print out current game string
                        print!("It's your turn | ");
                        if self.string.len() > 0 {
                            println!("{}", &self.string);
                        } else {
                            println!("None");
                        }

                        // assistance mode
                        if self.options.assist {
                            println!("Valid Options: {:?}", &valid_options);
                        }

                        loop {
                            let c = read_choice(&self.options.chars);
                            if !self.options.chars.contains(&c) {
                                printer::print_error(
                                    format!("{} is not a valid choice", c).as_str(),
                                );
                                continue;
                            }
                            if !valid_options.contains(&c) {
                                if self.options.assist {
                                    printer::print_error(
                                        format!("{} will make you lose!", c).as_str(),
                                    );
                                    continue;
                                } else {
                                    printer::print_loss_computer(player, start);
                                    printer::print_loss_stack(
                                        &self.string,
                                        self.options.substr_len,
                                        &c,
                                    );
                                    return 0; // success
                                }
                            }

                            self.string.push(c);
                            break;
                        }
                    } else {
                        // computer logic
                        let mut best_choice: char = '?';
                        let mut choices_count: u8 = 255;
                        for c in valid_options {
                            let choices = self.get_options(c);
                            if choices == 0 {
                                print_loss_computer(1, true);
                                return 0; // success
                            }
                            if choices < choices_count {
                                best_choice = c;
                                choices_count = choices;
                            }
                        }

                        self.string.push(best_choice);
                    }
                }
            }
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

                // print out current game string
                print!("Player {}'s turn | ", player);
                if self.string.len() > 0 {
                    println!("{}", &self.string);
                } else {
                    println!("None");
                }

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
                            printer::print_loss_stack(&self.string, self.options.substr_len, &c);
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

    #[test]
    fn get_options() {
        let mut opt = Options::default();
        opt.substr_len = 3;
        let g3 = Game {
            string: String::from("00010020"),
            options: opt,
        };
        assert_eq!(g3.get_options('0'), 0);
    }

    #[test]
    fn get_options_all_available() {
        let mut opt = Options::default();
        opt.substr_len = 3;
        let g4 = Game {
            string: String::from("0001002"),
            options: opt,
        };
        assert_eq!(g4.get_options('0'), 3);
    }
}
