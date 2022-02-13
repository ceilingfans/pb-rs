extern crate termion;

use termion::color::Red;
use termion::{clear, color, style};

pub fn print_prompt(s: &str) {
    println!(
        "{}{}{}{}",
        style::Bold,
        color::Fg(color::LightGreen),
        s,
        style::Reset,
    );
}

pub fn print_error(msg: &str) {
    println!(
        "{}{}error{}: {}",
        style::Bold,
        color::Fg(color::Red),
        style::Reset,
        msg,
    )
}

pub fn print_win(player: u8) {
    println!(
        "{}{}Victory!{} Player {} has won!",
        style::Bold,
        color::Fg(color::Yellow),
        style::Reset,
        player,
    )
}

pub fn print_loss(player: u8) {
    println!(
        "{}{}Game over!{} Player {} has lost D;",
        style::Bold,
        color::Fg(color::Red),
        style::Reset,
        player
    )
}

pub fn clear_screen() {
    println!("{}", clear::All);
}
