extern crate termion;

use std::collections::HashSet;
use substring::Substring;
// use termion::clear;

use crate::configuration::{options::*, reader::*};

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
    pub(crate) options: Options,
    pub(crate) string: String,
}

impl Game {
    pub fn new() -> Self {
        Game {
            options: get_options(),
            string: String::new(),
        }
    }

    pub fn get_valid_options(&self) -> HashSet<char> {
        if self.string.len() < self.options.substr_len as usize {
            return self.options.chars.clone();
        }

        let mut ret = HashSet::<char>::new();
        let prefix = self.string.substring(
            self.string.len() - (self.options.substr_len - 1) as usize,
            self.string.len(),
        );
        println!("{}", prefix);

        for ch in &self.options.chars {
            if !self.string.contains(format!("{}{}", prefix, ch).as_str()) {
                ret.insert(ch.clone());
            }
        }

        ret
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