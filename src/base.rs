// Base functionality

use std::str::CharIndices;

enum CharType {
    Letter,
    Number,
    Punctuation,
    Whitespace,
    NonAscii,
}

impl CharType {
    fn new(ch: char) -> CharType {
        if ch.is_ascii_punctuation() {
            CharType::Punctuation
        } else if ch.is_ascii_alphabetic() {
            CharType::Letter
        } else if ch.is_digit(10) {
            CharType::Number
        } else if ch.is_ascii_whitespace() {
            CharType::Whitespace
        } else {
            CharType::NonAscii
        }
    }

    // Returns true if the next character can fall after without breaking the
    // lexeme.
    fn can_append(&self, ch: char) -> bool {
        match self {
            CharType::Punctuation => ch.is_ascii_punctuation(),
            CharType::Letter => ch.is_ascii_alphanumeric(),
            CharType::Number => ch.is_ascii_alphanumeric(),
            CharType::Whitespace => ch.is_ascii_whitespace(),
            CharType::NonAscii => true,
        }
    }
}

/// A Lexeme
pub enum Lexeme<'a> {
    Word(&'a str),
    Number(&'a str),
    Punctuation(&'a str),
}

/// An iterator over lexemes in a file.
pub struct LexemeIterator<'a> {
    text: &'a str,
    chars: CharIndices<'a>,
    nextc: Option<(usize, char)>,
}

impl<'a> LexemeIterator<'a> {
    /// Create a new lexeme iterator from a string.
    pub fn new(text: &'a str) -> Self {
        let mut chars = text.char_indices();
        let nextc = chars.next();
        LexemeIterator { text, chars, nextc }
    }
}

impl<'a> Iterator for LexemeIterator<'a> {
    type Item = Lexeme<'a>;

    fn next(&mut self) -> Option<Lexeme<'a>> {
        let (start_index, ch) = self.nextc?;
        let kind = CharType::new(ch);
        let mut end_index = None;

        while let Some((i, ch)) = self.chars.next() {
            if !kind.can_append(ch) {
                end_index = Some(i);
                break;
            }
        }

        let slice = if let Some(end_index) = end_index {
            &self.text[start_index .. end_index]
        } else {
            &self.text[start_index ..]
        };

        match kind {
            CharType::Letter => Some(Lexeme::Word(slice)),
            CharType::Number => Some(Lexeme::Number(slice)),
            CharType::Punctuation => Some(Lexeme::Punctuation(slice)),
            CharType::Whitespace => Self::next(self),
            CharType::NonAscii => panic!("Failed to compile, invalid ascii!"),
        }
    }
}
