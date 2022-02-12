mod configuration;
mod util;

use crate::configuration::reader::{read_assist, read_chars, read_substr_len};
use configuration::reader::read_players;

fn main() {
    let players = read_players().unwrap();
    let substr_len = read_substr_len().unwrap();
    let assist = read_assist().unwrap();
    let chars = read_chars().unwrap();
    println!(
        "players: {}\nlen: {}\nassist: {}\nchars: {:?}",
        players, substr_len, assist, chars
    )
}
