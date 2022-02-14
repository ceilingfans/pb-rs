extern crate termion;

use termion::{clear, color, style};

use substring::Substring;

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
    );
}

pub fn print_win(player: u8) {
    println!(
        "{}{}Yay!{} Player {} has won!",
        style::Bold,
        color::Fg(color::Yellow),
        style::Reset,
        player,
    );
}

pub fn print_loss(player: u8) {
    println!(
        "{}{}Oh no!{} Player {} has lost D;",
        style::Bold,
        color::Fg(color::Red),
        style::Reset,
        player
    );
}

pub fn print_loss_computer(player: u8, start: bool) {
    // human
    if (player == 1 && start) || (player == 2 && start) {
        print_loss(player);
    } else {
        println!(
            "{}{}Yay!{} You have won!",
            style::Bold,
            color::Fg(color::Yellow),
            style::Reset,
        );
    }
}

pub fn clear_screen() {
    print!("{}", clear::All);
}

pub fn print_loss_stack(string: &String, substr_len: u8, ch: &char) {
    let prefix = string.substring(string.len() - (substr_len - 1) as usize, string.len());
    let matcher = format!("{}{}", prefix, ch);
    let pos = string.find(&matcher);
    if let Some(p) = pos {
        println!("{}", string);
        println!(
            "{}{}{}{}{} <-- {}{}{}{}{}{}",
            style::Bold,
            color::Fg(color::Red),
            " ".repeat(p),
            "^".repeat(matcher.len()),
            style::Reset,
            style::Bold,
            style::Underline,
            prefix,
            color::Fg(color::Red),
            ch,
            style::Reset
        );
    } else {
        print_error("internal error, could not get position in util::printer::print_loss_stack, please report this at https://github.com/ceilingfans/pb-rs/issues");
    }
}
