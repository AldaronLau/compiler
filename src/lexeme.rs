// Lexeme iterator.

use std::str::CharIndices;

enum CharType {
    Letter,
    Number,
    Bracket,
    Operator,
    Whitespace,
    NonAscii,
}

impl CharType {
    fn new(ch: char) -> CharType {
        if ch.is_ascii_alphabetic() || ch == '_' {
            CharType::Letter
        } else if ch == '(' || ch == ')' || ch == '[' || ch == ']'
            || ch == '{' || ch == '}'
        {
            CharType::Bracket
        } else if ch.is_digit(10) {
            CharType::Number
        } else if ch.is_ascii_punctuation() {
            CharType::Operator
        } else if ch.is_ascii_whitespace() {
            CharType::Whitespace
        }else {
            CharType::NonAscii
        }
    }

    // Returns true if the next character can fall after without breaking the
    // lexeme.
    fn can_append(&self, ch: char) -> bool {
        match self {
            CharType::Bracket => false,
            CharType::Operator => ch.is_ascii_punctuation() && ch != '_'
                && ch != '(' && ch != ')' && ch != '[' && ch != ']'
                && ch != '{' && ch != '}',
            CharType::Letter => ch.is_ascii_alphanumeric() || ch == '_',
            CharType::Number => ch.is_ascii_alphanumeric() || ch == '_',
            CharType::Whitespace => ch.is_ascii_whitespace(),
            CharType::NonAscii => true,
        }
    }
}

/// A Lexeme
#[derive(Debug)]
pub enum Lexeme<'a> {
    /// An identifier or keyword
    Word(&'a str),
    /// A number
    Number(&'a str),
    /// A string of punctuation, may or may not be interpreted separately
    ///
    /// Example: `a::<B::<C>>::new() >> d`; `>>` will always count as 1 lexeme
    /// even though it's first occurance it should be interpreted as 2.
    Operator(&'a str),
    /// One of: `()[]{}`
    Bracket(&'a str),
    /// A comment, string or character
    Text(&'a str),
}

impl<'a> std::fmt::Display for Lexeme<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Lexeme::Word(text) => write!(f, "word \"{}\"", text),
            Lexeme::Number(text) => write!(f, "number: \"{}\"", text),
            Lexeme::Operator(text) => write!(f, "operator: \"{}\"", text),
            Lexeme::Bracket(text) => write!(f, "bracket: \"{}\"", text),
            Lexeme::Text(text) => write!(f, "text: \"{}\"", text),
        }
    }
}

/// An iterator over lexemes in a file.
pub struct LexemeIterator<'a, C> {
    text: &'a str,
    chars: CharIndices<'a>,
    nextc: Option<(usize, char)>,
    begin_text: fn(&str) -> (Option<C>, Option<char>),
    end_text: fn(&str, &C) -> (bool, usize),
}

impl<'a, C> LexemeIterator<'a, C> {
    /// Create a new lexeme iterator from a string.
    pub fn new(text: &'a str, begin_text: fn(&str) -> (Option<C>, Option<char>), end_text: fn(&str, &C) -> (bool, usize)) -> Self {
        let mut chars = text.char_indices();
        let nextc = chars.next();
        LexemeIterator { text, chars, nextc, begin_text, end_text }
    }
}

impl<'a, C> Iterator for LexemeIterator<'a, C> {
    type Item = Lexeme<'a>;

    fn next(&mut self) -> Option<Lexeme<'a>> {
        let (start_index, ch) = self.nextc?;
        let mut end_index = None;
        let (chunk_kind, _escape) = (self.begin_text)(&self.text[start_index..]);

        self.nextc = None;

        let kind = if let Some(ref chunk) = chunk_kind {
            let kind = CharType::NonAscii;

            while let Some((i, ch)) = self.chars.next() {
                let (end, end_size) = (self.end_text)(&self.text[i..], chunk);

                let quit = if end {
                    for _ in 0..end_size {
                        self.nextc = self.chars.next();
                    }
                    true
                } else if !kind.can_append(ch) {
                    self.nextc = Some((i, ch));
                    true
                } else {
                    false
                };
                if quit {
                    end_index = Some(i);
                    break;
                }
            }

            kind
        } else {
            let kind = CharType::new(ch);

            while let Some((i, ch)) = self.chars.next() {
                let (chunk, _esc) = (self.begin_text)(&self.text[i..]);

                if !kind.can_append(ch) || chunk.is_some() {
                    self.nextc = Some((i, ch));
                    end_index = Some(i);
                    break;
                }
            }

            kind
        };

        let slice = if let Some(end_index) = end_index {
            &self.text[start_index .. end_index]
        } else {
            &self.text[start_index ..]
        };

        match kind {
            CharType::Letter => Some(Lexeme::Word(slice)),
            CharType::Number => Some(Lexeme::Number(slice)),
            CharType::Operator => Some(Lexeme::Operator(slice)),
            CharType::Bracket => Some(Lexeme::Bracket(slice)),
            CharType::Whitespace => Self::next(self),
            CharType::NonAscii => if chunk_kind.is_some() {
                Some(Lexeme::Text(slice))
            } else {
                panic!("Failed to compile, invalid ascii!")                
            }
        }
    }
}
