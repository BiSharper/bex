pub trait LexicalScope : Default {

}

#[derive(Default)]
pub struct NoLexicalScope;

impl LexicalScope for NoLexicalScope {}
