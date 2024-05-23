use std::ops::{Deref};
use crate::BErrorScoped;
use crate::source::base::{BKnownStartSource, BSourceBase, BSourceMeta};
use crate::source::BDynamicSource;

pub trait BPeekableSource<T>: BDynamicSource<T> {
    fn peek(&mut self) -> Result<(Self::Offset, Option<Self::Token>), Self::Error>;

    fn b_has_peeked(&self) -> bool;

    fn b_dump_peeked(&mut self) -> Option<(Self::Offset, Option<Self::Token>)>;
}

pub struct BIntoPeekable<T: Clone, S: BDynamicSource<T>> {
    inner: S,
    next: Option<(S::Offset, Option<S::Token>)>
}

impl<T: Clone, S: BDynamicSource<T>> From<S> for BIntoPeekable<T, S>  {
    fn from(value: S) -> Self { Self { inner: value, next: None } }
}

impl<T: Clone, S: BDynamicSource<T>> Deref for BIntoPeekable<T, S> {
    type Target = S;

    fn deref(&self) -> &Self::Target { &self.inner }
}

impl<T: Clone, S: BDynamicSource<T>> BDynamicSource<T> for BIntoPeekable<T, S>
    where S::Token: Clone
{
    fn b_position(&self) -> Self::Offset { self.inner.b_position() }

    fn b_seek(&mut self, offset: Self::Offset) -> Result<(), Self::Error> {
        self.inner.b_seek(offset)
    }

    fn b_traversed(&self) -> Self::Offset {
        match &self.next {
            None => self.b_position(),
            Some((offset, _)) => self.b_position() + *offset
        }
    }

    fn b_next(&mut self) -> Result<Option<Self::Token>, Self::Error> {
        if self.next.is_none() {
            return self.inner.b_next()
        }

        let (next_offset, next_token) = self.b_dump_peeked().unwrap();
        self.b_seek(next_offset)?;
        return Ok(next_token)

    }
}

impl<T: Clone, S: BDynamicSource<T>> BSourceBase<T> for BIntoPeekable<T, S> { type Token = S::Token; }

impl<T: Clone, S: BDynamicSource<T>> BErrorScoped for BIntoPeekable<T, S> { type Error = S::Error; }

impl<T: Clone, S: BDynamicSource<T>> BSourceMeta for BIntoPeekable<T, S> { type Offset = S::Offset; }

impl<T: Clone, S: BDynamicSource<T>> BKnownStartSource for BIntoPeekable<T, S> {
    fn b_start(&self) -> Self::Offset { self.inner.b_start() }
}

impl<T: Clone, S: BDynamicSource<T>> BPeekableSource<T> for BIntoPeekable<T, S>
    where S::Token: Clone
{
    fn peek(&mut self) -> Result<(Self::Offset, Option<Self::Token>), Self::Error> {
        if let Some((offset, ref token)) = self.next {
            return Ok((offset, token.clone()));
        }
        let start_offset = self.b_position();
        let next = self.b_next()?;
        let next_offset = self.b_position();
        self.b_seek(start_offset)?;
        self.next = Some((next_offset, next.clone()));
        Ok((next_offset, next))
    }

    fn b_has_peeked(&self) -> bool { self.next.is_some() }

    fn b_dump_peeked(&mut self) -> Option<(Self::Offset, Option<Self::Token>)> { self.next.take() }
}