// Lexeme parsing

use yeet;

enum RustChunk {
    SingleLineComment,
    MultiLineComment,
    String,
    Character,
    RawString(usize),
}

fn main() {
    let file = std::fs::read_to_string("src/lexeme.rs").unwrap();

    fn rs_open(input: &str) -> (Option<RustChunk>, Option<char>) {
        if input.starts_with("//") {
            (Some(RustChunk::SingleLineComment), Some('\\'))
        } else if input.starts_with("/*") {
            (Some(RustChunk::MultiLineComment), None)
        } else if input.starts_with("\"") {
            (Some(RustChunk::String), Some('\\'))
        } else if input.starts_with("'") {
            (Some(RustChunk::Character), Some('\\'))
        } else if input.starts_with("r#") {
            if let Some(index) = input[2..].find("\"") {
                (Some(RustChunk::RawString(index)), None)
            } else {
                (None, None)
            }
        } else {
            (None, None)
        }
    };

    fn rs_close(input: &str, chunk: &RustChunk) -> (bool, usize) {
        match chunk {
            RustChunk::SingleLineComment => (input.starts_with("\n"), 1),
            RustChunk::MultiLineComment => (input.starts_with("*/"), 2),
            RustChunk::String => (input.starts_with("\""), 1),
            RustChunk::Character => (input.starts_with("'"), 1),
            RustChunk::RawString(extra) => (if input.starts_with("\"#") {
                Some(*extra) == input[2..].find(|c| c != '#')
            } else {
                false
            }, 2 + extra)
        }
    };

    let mut it = yeet::LexemeIterator::new(&file, rs_open, rs_close);

    for lexeme in it {
        println!("{}", lexeme);
    }
}
