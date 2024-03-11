use std::fmt::Debug;
use std::io;
use std::io::{Read, Seek};
use crate::{Bexer, Tokenizer};

pub trait Parse: Sized {
    type ParseError: From<io::Error> + Debug;

    fn try_parse<R: Read + Seek>(filename: &str, lexer: Bexer<R>) -> Result<Self, Self::ParseError>;

    fn parse<R: Read + Seek>(filename: &str, lexer: Bexer<R>) -> Self {
        Self::try_parse(filename, lexer).unwrap()
    }
}