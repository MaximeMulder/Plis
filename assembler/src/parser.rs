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

    pub fn next_word(&mut self) -> Option<&str> {
        self.next();
        self.word()
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
            if character.is_alphanumeric() {
                break;
            }

            self.advance(character);
        }
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
