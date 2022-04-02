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
