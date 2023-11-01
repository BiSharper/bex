use std::error::Error;
use std::io;
use crate::{Lexer, Token};

/// The `PreProcess` trait defines the methods required to preprocess the lexers content before parsing
///
/// # Type Parameters
/// * `T` - Any type that is Sized (has a constant size in memory), and can be compared for equality.
pub trait PreProcess<T: Sized + PartialEq + Copy> {
    type E: Error + From<io::Error>;
    /// Does preprocessing on the given lexer
    ///
    /// # Arguments
    ///
    /// * `lexer` - The lexer whose content is to be preprocessed
    fn preprocess(&mut self, lexer: Lexer<T>) -> Result<Lexer<T>, Self::E>;
}