use std::collections::HashMap;

pub struct Parser<'a> {
    text: &'a str,
    cursor: usize,
}

impl<'a> Parser<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text, cursor: 0 }
    }

    pub fn next_word(&mut self) -> Option<&str> {
        let mut lookahead = self.lookahead();
        loop {
            let Some(character) = lookahead else {
                return None;
            };

            if character.is_alphanumeric() {
                break;
            }

            self.advance();
            lookahead = self.lookahead();
        }

        let mut length = 0;
        while let Some(character) = self.lookahead() {
            if !character.is_alphanumeric() {
                break;
            }

            length += self.advance();
        }

        Some(&self.text[self.cursor - length..self.cursor])
    }

    pub fn expect_const8(&mut self, labels: &HashMap<Box<str>, usize>) -> [u8; 1] {
        let word = self.next_word().unwrap();
        let constant = word.parse::<u8>().unwrap_or_else(|_| {
            labels.get(word).unwrap().clone().try_into().unwrap()
        });

        constant.to_be_bytes()
    }

    pub fn expect_const16(&mut self, labels: &HashMap<Box<str>, usize>) -> [u8; 2] {
        let word = self.next_word().unwrap();
        let constant = word.parse::<u16>().unwrap_or_else(|_| {
            labels.get(word).unwrap().clone().try_into().unwrap()
        });

        constant.to_be_bytes()
    }

    pub fn expect_const32(&mut self, labels: &HashMap<Box<str>, usize>) -> [u8; 4] {
        let word = self.next_word().unwrap();
        let constant = word.parse::<u32>().unwrap_or_else(|_| {
            labels.get(word).unwrap().clone().try_into().unwrap()
        });

        constant.to_be_bytes()
    }

    pub fn expect_const64(&mut self, labels: &HashMap<Box<str>, usize>) -> [u8; 8] {
        let word = self.next_word().unwrap();
        let constant = word.parse::<u64>().unwrap_or_else(|_| {
            labels.get(word).unwrap().clone().try_into().unwrap()
        });

        constant.to_be_bytes()
    }

    pub fn expect_register(&mut self) -> u8 {
        let word = self.next_word();
        let constant = word.unwrap().parse::<u8>().unwrap();
        constant.to_be()
    }

    pub fn expect_lock(&mut self) -> u8 {
        let word = self.next_word();
        let constant = word.unwrap().parse::<u8>().unwrap();
        constant.to_be()
    }

    pub fn expect_thread(&mut self) -> u8 {
        let word = self.next_word();
        let constant = word.unwrap().parse::<u8>().unwrap();
        constant.to_be()
    }
}

impl<'a> Parser<'a> {
    fn lookahead(&self) -> Option<char> {
        self.text[self.cursor..].chars().next()
    }

    fn advance(&mut self) -> usize {
        let length = self.lookahead().unwrap().len_utf8();
        self.cursor += length;
        length
    }
}
