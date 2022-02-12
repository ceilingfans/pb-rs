extern crate termion;

use termion::{color, style};

pub fn format_prompt(s: &str) -> String {
    format!(
        "{}{}{}{}",
        style::Bold,
        color::Fg(color::LightGreen),
        s,
        style::Reset,
    )
}

pub fn format_error(msg: &str) -> String {
    format!(
        "{}{}error{}: {}",
        style::Bold,
        color::Fg(color::Red),
        style::Reset,
        msg,
    )
}
