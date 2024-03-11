use std::fmt::Debug;
use std::io;
use std::io::{Read, Seek};
use crate::{Bexer, LexicalScope};

pub trait Token: Sized + PartialEq + Clone {

    fn is_eof(&self) -> bool;
}

pub trait Tokenizer {
    type Error: From<io::Error> + Debug;
    type Scope: LexicalScope;
    type Token: Token;

    fn lex<R: Read + Seek>(
        lexer: &Bexer<R>,
        scope: &mut Self::Scope
    ) -> Self::Token {
        Self::try_lex(lexer, scope).unwrap()
    }

    fn try_lex<R: Read + Seek>(
        lexer: &Bexer<R>,
        scope: &mut Self::Scope
    ) -> Result<Self::Token, Self::Error>;

    fn try_tokenize_fully<R: Read + Seek> (
        lexer: Bexer<R>,
        scope: &mut Self::Scope
    ) -> Result<Vec<Self::Token>, Self::Error> {
        let mut tokens = vec![];

        loop {
            let token = Self::try_lex(&lexer, scope)?;
            if token.is_eof() { break; }
            tokens.push(token)
        }

        Ok(tokens)
    }

    fn tokenize_fully<R: Read + Seek> (
        lexer: Bexer<R>,
        scope: &mut Self::Scope
    ) -> Vec<Self::Token> {
        Self::try_tokenize_fully(lexer, scope).unwrap()
    }
}