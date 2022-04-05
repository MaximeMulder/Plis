use std::process::exit;

pub type ParserResult<T> = Result<T, Box<str>>;

pub struct Parser<'a> {
    text: &'a str,
    cursor: usize,
}

impl<'a> Parser<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            cursor: 0,
        }
    }

    pub fn next_comma(&mut self) -> bool {
        self.next();
        self.comma()
    }

    pub fn next_colon(&mut self) -> bool {
        self.next();
        self.colon()
    }

    pub fn next_word(&mut self) -> Option<&str> {
        self.next();
        self.word()
    }

    pub fn error(&self, message: &str) -> ! {
        println!("ERROR POSITION {}: {}", self.cursor, message);
        exit(0);
    }
}

impl<'a> Parser<'a> {
    fn lookahead(&self) -> Option<char> {
        self.text[self.cursor..].chars().next()
    }

    fn advance(&mut self, character: char) -> usize {
        let length = character.len_utf8();
        self.cursor += length;
        length
    }

    fn next(&mut self) {
        while let Some(character) = self.lookahead() {
            if !character.is_whitespace() {
                break;
            }

            self.advance(character);
        }
    }

    fn comma(&mut self) -> bool {
        let Some(character) = self.lookahead() else {
            return false;
        };

        if character != ',' {
            return false;
        }

        self.advance(character);
        true
    }

    fn colon(&mut self) -> bool {
        let Some(character) = self.lookahead() else {
            return false;
        };

        if character != ':' {
            return false;
        }

        self.advance(character);
        true
    }

    fn word(&mut self) -> Option<&str> {
        let mut length = 0;
        while let Some(character) = self.lookahead() {
            if !character.is_alphanumeric() {
                break;
            }

            length += self.advance(character);
        }

        if length == 0 {
            return None;
        }

        Some(&self.text[self.cursor - length .. self.cursor])
    }
}
