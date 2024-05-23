use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use crate::BErrorScoped;
use crate::source::{BDynamicSource, BStaticSource};
use crate::source::base::{BKnownStartSource, BSourceBase, BSourceMeta};

pub struct BIntoDynamic<T, S: BStaticSource<T>> {
    inner: S,
    index: S::Offset,
    _phantom: PhantomData<T>
}

impl<T, S: BStaticSource<T>> BIntoDynamic<T, S> {
    pub fn create(inner: S, index: S::Offset) -> Self {
        Self { inner, index, _phantom: PhantomData, }
    }
}

impl<T, S: BStaticSource<T>> From<S> for BIntoDynamic<T, S> {
    fn from(value: S) -> Self {
        let start = value.b_start();
        Self::create(value, start)
    }
}

impl<T, S: BStaticSource<T>> Deref for BIntoDynamic<T, S> {
    type Target = S;

    fn deref(&self) -> &Self::Target { &self.inner }
}
impl<T, S: BStaticSource<T>> DerefMut for BIntoDynamic<T, S> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.inner }
}

impl<T, S: BStaticSource<T>> BSourceBase<T> for BIntoDynamic<T, S> {
    type Token = S::Token;
}

impl<T, S: BStaticSource<T>> BErrorScoped for BIntoDynamic<T, S> {
    type Error = S::Error;
}

impl<T, S: BStaticSource<T>> BSourceMeta for BIntoDynamic<T, S> {
    type Offset = S::Offset;
}

impl<T, S: BStaticSource<T>> BKnownStartSource for BIntoDynamic<T, S> {
    fn b_start(&self) -> Self::Offset {
        self.inner.b_start()
    }
}

impl<T, S: BStaticSource<T>> BDynamicSource<T> for BIntoDynamic<T, S> {
    fn b_position(&self) -> Self::Offset { self.index }

    fn b_seek(&mut self, offset: Self::Offset) -> Result<(), Self::Error> {
        self.index = offset;
        Ok(())
    }

    fn b_next(&mut self) -> Result<Option<Self::Token>, Self::Error> {
        let (next_position, next_token) = self.inner.b_next_at(self.index)?;
        self.b_seek(next_position)?;
        return Ok(next_token)
    }
}