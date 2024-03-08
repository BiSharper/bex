use std::io::{Read, Seek};
use crate::Bexer;

trait Tokenizer : Sized + PartialEq + Clone {
    type LexicalError;

    fn lexeme<R: Read + Seek>(lexer: &Bexer<R>) -> Result<Self, Self::LexicalError>;
}

trait ScopedTokenizer: Tokenizer {
    type LexicalScope: Default;
    fn scoped_lexeme<R: Read + Seek>(lexer: &Bexer<R>, scope: &Self::LexicalScope) -> Result<Self, Self::LexicalError>;
}

impl<ST: ScopedTokenizer> Tokenizer for ST {
    type LexicalError = Self::LexicalError;

    fn lexeme<R: Read + Seek>(lexer: &Bexer<R>) -> Result<Self, Self::LexicalError> {
        Self::scoped_lexeme(lexer, &Default::default())
    }
}