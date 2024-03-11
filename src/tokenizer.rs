use std::fmt::Debug;
use std::io;
use std::io::{Read, Seek};
use crate::{Bexer, LexicalScope};

pub trait Token: Sized + PartialEq + Clone {
    fn length(&self) -> usize;

    fn position(&self) -> usize;

    fn is_eof(&self) -> bool;
}

pub trait Tokenizer {
    type Error: From<io::Error> + Debug;
    type Token: Token;

    fn lex<R: Read + Seek>(
        lexer: &mut Bexer<R>
    ) -> Self::Token {
        Self::try_lex(lexer).unwrap()
    }

    fn try_lex<R: Read + Seek>(
        lexer: &mut Bexer<R>
    ) -> Result<Self::Token, Self::Error>;

}

pub trait ScopedTokenizer : Tokenizer<
    Error = <Self as ScopedTokenizer>::Error,
    Token = <Self as ScopedTokenizer>::Token
> {
    type Error: From<io::Error> + Debug;
    type Scope: LexicalScope;
    type Token: Token;

    fn lex_scoped<R: Read + Seek>(
        lexer: &Bexer<R>,
        scope: &mut Self::Scope
    ) -> <Self as Tokenizer>::Token {
        Self::try_lex_scoped(lexer, scope).unwrap()
    }

    fn try_lex_scoped<R: Read + Seek>(
        lexer: &Bexer<R>,
        scope: &mut Self::Scope
    ) -> Result<<Self as Tokenizer>::Token, <Self as Tokenizer>::Error>;
}

impl<
    TScopedTokenizer: ScopedTokenizer
> Tokenizer for TScopedTokenizer {
    type Error = <Self as ScopedTokenizer>::Error;
    type Token = <Self as ScopedTokenizer>::Token;

    fn try_lex<R: Read + Seek>(lexer: &Bexer<R>) -> Result<Self::Token, Self::Error> {
        Self::try_lex_scoped(lexer, &mut Default::default())
    }
}
