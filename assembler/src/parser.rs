use std::process::exit;

pub type ParserResult<T> = Result<T, Box<str>>;

pub struct Parser<'a> {
    text: &'a str,
    previous: usize,
    cursor: usize,
}

impl<'a> Parser<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            previous: 0,
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
        println!("ERROR POSITION {}: {}", self.previous, message);
        exit(0);
    }
}

impl<'a> Parser<'a> {
    fn lookahead(&self) -> Option<char> {
        self.text[self.cursor..].chars().next()
    }

    fn advance(&mut self, character: char) {
        self.cursor += character.len_utf8();
    }

    fn next(&mut self) {
        while let Some(character) = self.lookahead() {
            if !character.is_whitespace() {
                break;
            }

            self.advance(character);
        }

        self.previous = self.cursor;
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
        while let Some(character) = self.lookahead() {
            if !character.is_alphanumeric() {
                break;
            }

            self.advance(character);
        }

        if self.cursor == self.previous {
            return None;
        }

        Some(&self.text[self.previous .. self.cursor])
    }
}
