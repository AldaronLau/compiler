// C
//
//! C Programming Language version [C2x](https://en.wikipedia.org/wiki/C2x)
//! without platform-dependant sizes (int is typedef for int32_t, etc).

use crate::{LexemeIterator, Lexeme};

/// A C Keyword
pub enum Keyword {
    Auto,
    Double,
    Int,
    Struct,
    Break,
    Else,
    Long,
    Switch,
    Case,
    Enum,
    Register,
    Typedef,
    Char,
    Extern,
    Return,
    Union,
    Const,
    Float,
    Short,
    Unsigned,
    Continue,
    For,
    Signed,
    Void,
    Default,
    Goto,
    Sizeof,
    Volatile,
    Do,
    If,
    Static,
    While,
}

/// A C Built-in type
pub enum BuiltInType {
    /// No-size type
    Void,
    /// \>= 8 bits. => char8_t
    Char,
    /// Usually 32 bits. => float32_t
    Float,
    /// Usually 64 bits. => float64_t
    Double,
    /// \>= 8 bits => int8_t
    SignedChar,
    /// \>= 16 bits => int16_t
    SignedShort,
    /// \>= 16 bits (usually 32) => int32_t
    SignedInt,
    /// \>= 32 bits => ssize_t
    SignedLongInt,
    /// \>= 64 bits => int64_t
    SignedLongLongInt,
    /// \>= 8 bits => uint8_t
    UnsignedChar,
    /// \>= 16 bits => uint16_t
    UnsignedShort,
    /// \>= 16 bits (usually 32) => uint32_t
    UnsignedInt,
    /// \>= 32 bits => size_t
    UnsignedLongInt,
    /// \>= 64 bits => uint64_t
    UnsignedLongLongInt,
    /// \>= 8 bits => Bool
    _Bool,
    /// TODO => Complex
    _Complex,
    /// TODO => Complex
    _Imaginary,

    // Yeet Extension built-in types
    Float16T, // f16
    Float32T, // f32
    Float64T, // f64
    Float80T, // f80
    Int8T, // i8
    Int16T, // i16
    Int32T, // i32
    Int64T, // i64
    Int128T, // i128
    Uint8T, // u8
    Uint16T, // u16
    Uint32T, // u32
    Uint64T, // u64
    Uint128T, // u128
    SsizeT, // isize
    SizeT, // usize
    Bool, // bool
    Complex, // Complex
    Imaginary, // Imaginary
    Char8T, // u8 (ascii character, or part of unicode codepoint)
}

/// A C Type
pub enum Type<'a> {
    /// A built-in type
    BuiltIn(BuiltInType),
    /// A struct preceded by the struct keyword or is an enum.
    Defined(&'a str),
    /// A typedef for either a built-in type, struct or enum.  Needs to be
    /// resolved.
    Typedef(&'a str),
}

/// Variable definition.
pub struct Variable<'a> {
    pub ty: Type<'a>,
    pub name: &'a str,
}

/// A prototype for a function.  May have a block ('{') or a `;`.
pub struct Prototype<'a> {
    // The first component of a prototype.
    pub return_type: Type<'a>,
    // Name of the function
    pub name: &'a str,
    // Formal parameters to the function
    pub params: Vec<Variable<'a>>,
    // A code block to define what the function does.
    pub block: Option<Block<'a>>,
}

enum CChunk {
    SingleLineComment,
    MultiLineComment,
    String,
    Character,
}

/// An iterator over C tokens.
pub struct TokenIterator<'a> {
    lexemes: LexemeIterator<'a, CChunk>,
}

impl<'a> TokenIterator<'a> {
    /// Create a new C token iterator.
    pub fn new(text: &'a str) -> Self {
        let lexemes = LexemeIterator::new(text, begin_text, end_text);

        TokenIterator {
            lexemes
        }
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(match self.lexemes.next()? {
            Lexeme::Word(string) => match string {
                "auto" => Token::Keyword(Keyword::Auto),
                "double" => Token::Keyword(Keyword::Double),
                "int" => Token::Keyword(Keyword::Int),
                "struct" => Token::Keyword(Keyword::Struct),
                "break" => Token::Keyword(Keyword::Break),
                "else" => Token::Keyword(Keyword::Else),
                "long" => Token::Keyword(Keyword::Long),
                "switch" => Token::Keyword(Keyword::Switch),
                "case" => Token::Keyword(Keyword::Case),
                "enum" => Token::Keyword(Keyword::Enum),
                "register" => Token::Keyword(Keyword::Register),
                "typedef" => Token::Keyword(Keyword::Typedef),
                "char" => Token::Keyword(Keyword::Char),
                "extern" => Token::Keyword(Keyword::Extern),
                "return" => Token::Keyword(Keyword::Return),
                "union" => Token::Keyword(Keyword::Union),
                "const" => Token::Keyword(Keyword::Const),
                "float" => Token::Keyword(Keyword::Float),
                "short" => Token::Keyword(Keyword::Short),
                "unsigned" => Token::Keyword(Keyword::Unsigned),
                "continue" => Token::Keyword(Keyword::Continue),
                "for" => Token::Keyword(Keyword::For),
                "signed" => Token::Keyword(Keyword::Signed),
                "void" => Token::Keyword(Keyword::Void),
                "default" => Token::Keyword(Keyword::Default),
                "goto" => Token::Keyword(Keyword::Goto),
                "sizeof" => Token::Keyword(Keyword::Sizeof),
                "volatile" => Token::Keyword(Keyword::Volatile),
                "do" => Token::Keyword(Keyword::Do),
                "if" => Token::Keyword(Keyword::If),
                "static" => Token::Keyword(Keyword::Static),
                "while" => Token::Keyword(Keyword::While),
                id => Token::Identifier(id),
            },
            Lexeme::Text(string) => {
                if string.starts_with("//") {
                    Token::SingleLineComment(&string[2..])
                } else if string.starts_with("/*") {
                    Token::MultiLineComment(&string[2..])
                } else if string.starts_with("'") {
                    if string.len() != 2 {
                        if string.len() != 3 || string.chars().nth(1).unwrap() != '\\' {
                            panic!("Error: 1 unicode character expected in single quotes");
                        }
                        // Escaped characters
                        let ch = string.chars().nth(2).unwrap();
                        if ch != 'n' && ch != '0' && ch != 'r'
                            && ch != 'v' /* FIXME: there's more */
                        {
                            panic!("Invalid escape sequence");
                        }
                        Token::Character(ch)
                    } else {
                        // Non-escaped characters
                        Token::Character(string.chars().nth(1).unwrap())
                    }
                } else if string.starts_with("\"") {
                    Token::String(&string[1..])
                } else {
                    panic!("Compiler Bug: Invalid text")
                }
            },
            Lexeme::Number(text) => {
                let mut iter = text.split('.');

                let integer = iter.next().unwrap(); // FIXME.
                let integer = integer.parse::<i128>().unwrap(); // FIXME

                if let Some(mantissa) = iter.next() {
                    let mantissa = mantissa.parse::<u128>().unwrap(); // FIXME

                    Token::Float(integer, mantissa)
                } else {
                    Token::Int(integer)
                }
            }
            Lexeme::Operator(text) => {
                Token::Operator(match text {
                    "," => Operator::Separator,
                    ";" => Operator::Semicolon,
                    _ => panic!("Unknown operator"), // FIXME
                })
            }
            Lexeme::Bracket(text) => {
                Token::Bracket(match text {
                    "(" => Bracket::ParensL,
                    ")" => Bracket::ParensR,
                    "{" => Bracket::BraceL,
                    "}" => Bracket::BraceR,
                    "[" => Bracket::SquareL,
                    "]" => Bracket::SquareR,
                    _ => panic!("COMPILER BUG: Invalid bracket"),
                })
            }
        })
    }
}

pub enum Operator {
    Separator,
    Semicolon,
}

pub enum Bracket {
    ParensL,
    ParensR,
    BraceL,
    BraceR,
    SquareL,
    SquareR,
}

pub enum Token<'a> {
    Keyword(Keyword),
    MultiLineComment(&'a str),
    SingleLineComment(&'a str),
    Character(char),
    String(&'a str),
    Identifier(&'a str),
    Float(i128, u128),
    Int(i128),
    Operator(Operator),
    Bracket(Bracket),
}

pub enum Item<'a> {
    Prototype(Prototype<'a>),
    Block(Block<'a>),
}

pub struct ItemIterator<'a> {
    tokens: TokenIterator<'a>,
}

impl<'a> ItemIterator<'a> {
    pub fn new(text: &'a str) -> Self {
        ItemIterator {
            tokens: TokenIterator::new(text),
        }
    }
}

impl<'a> Iterator for ItemIterator<'a> {
    type Item = Result<Item<'a>, ()>;

    #[allow(unused)] // FIXME
    fn next(&mut self) -> Option<Self::Item> {
        let mut ty = None;
//        let var_definition = None;
//        let prototype_definition = None;

        match self.tokens.next()? {
            Token::Keyword(keyword) => match keyword {
                Keyword::Auto => todo!(),
                Keyword::Double => {
                    ty = Some(Type::BuiltIn(BuiltInType::Double));
                    let name = if let Some(token) = self.tokens.next() {
                        match token {
                            Token::Identifier(ident) => {
                                ident
                            }
                            _ => return Some(Err(()))
                        }
                    } else {
                        return Some(Err(()));
                    };
                    if let Some(token) = self.tokens.next() {
                        match token {
                            Token::Bracket(Bracket::ParensL) => { /* open */ }
                            _ => return Some(Err(()))
                        }
                    } else {
                        return Some(Err(()));
                    };
                },
                Keyword::Int => {
                },
                Keyword::Struct => {
                },
                Keyword::Break => {
                },
                Keyword::Else => {
                },
                Keyword::Long => {
                },
                Keyword::Switch => {
                },
                Keyword::Case => {
                },
                Keyword::Enum => {
                },
                Keyword::Register => {
                },
                Keyword::Typedef => {
                },
                Keyword::Char => {
                },
                Keyword::Extern => {
                },
                Keyword::Return => {
                },
                Keyword::Union => {
                },
                Keyword::Const => {
                },
                Keyword::Float => {
                },
                Keyword::Short => {
                },
                Keyword::Unsigned => {
                },
                Keyword::Continue => {
                },
                Keyword::For => {
                },
                Keyword::Signed => {
                },
                Keyword::Void => {
                },
                Keyword::Default => {
                },
                Keyword::Goto => {
                },
                Keyword::Sizeof => {
                },
                Keyword::Volatile => {
                },
                Keyword::Do => {
                },
                Keyword::If => {
                },
                Keyword::Static => {
                },
                Keyword::While => {
                },
            },
            Token::MultiLineComment(comment) => {},
            Token::SingleLineComment(comment) => {},
            Token::Character(ch) => {},
            Token::String(text) => {},
            Token::Identifier(ident) => {},
            Token::Float(int, mantissa) => {},
            Token::Int(int) => {},
            Token::Operator(op) => {},
            Token::Bracket(br) => {},
        };
        todo!() // FIXME
    }
}

pub struct Block<'a> {
    items: Vec<Item<'a>>,
}

fn begin_text(input: &str) -> (Option<CChunk>, Option<char>) {
    if input.starts_with("//") {
        (Some(CChunk::SingleLineComment), Some('\\'))
    } else if input.starts_with("/*") {
        (Some(CChunk::MultiLineComment), None)
    } else if input.starts_with("\"") {
        (Some(CChunk::String), Some('\\'))
    } else if input.starts_with("'") {
        (Some(CChunk::Character), Some('\\'))
    } else {
        (None, None)
    }
}

fn end_text(input: &str, chunk: &CChunk) -> (bool, usize) {
    match chunk {
        CChunk::SingleLineComment => (input.starts_with("\n"), 1),
        CChunk::MultiLineComment => (input.starts_with("*/"), 2),
        CChunk::String => (input.starts_with("\""), 1),
        CChunk::Character => (input.starts_with("'"), 1),
    }
}
