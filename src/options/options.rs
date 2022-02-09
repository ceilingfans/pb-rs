use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct Options {
    pub players: u8,
    pub substr_len: u8,
    pub assist: bool,
    pub chars: HashSet<char>,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            players: 2,
            substr_len: 4,
            assist: false,
            chars: HashSet::from(['0', '1', '2']),
        }
    }
}

#[cfg(Test)]
mod tests {
    use super::*;

    #[test]
    fn it_generates_default() {
        let default_options = Options {
            players: 2,
            substr_len: 4,
            assist: false,
            chars: HashSet::from(['0', '1', '2']),
        };

        assert_eq!(Options::default(), default_options);
    }
}