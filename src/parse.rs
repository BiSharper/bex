use std::fmt::Debug;
use std::io;
use crate::lexer::{Lexer, Token};

/// The `Parse` trait defines the methods required to parse the lexers content or tokens
///
/// # Type Parameters
/// * `T` - Any type that is Sized (has a constant size in memory), and can be compared for equality.
pub trait Parse<T: Sized + PartialEq + Copy>: Sized {
    type E: From<io::Error> + Debug;

    /// Parses the given file using the given lexer and returns the parser.
    /// This method will panic in case of any errors during parsing.
    ///
    /// # Arguments
    ///
    /// * `filename` - The input file to parse
    /// * `lexer` - The lexer to use for parsing
    fn parse(filename: String, lexer: &mut Lexer<T>) -> Self { Self::try_parse(filename, lexer).unwrap() }

    /// Attempts to parse the given file using the given lexer and returns the parser or an error.
    ///
    /// # Arguments
    ///
    /// * `filename` - The input file to parse
    /// * `lexer` - The lexer to use for parsing
    fn try_parse(filename: String, lexer: &mut Lexer<T>) -> Result<Self, Self::E>;
}
