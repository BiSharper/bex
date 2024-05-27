use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Range};
use crate::BErrorScoped;
use crate::source::{BDynamicSource, BDynamicSourceBase, BKnownEndSource, BSeekableSource, BSliceableSource, BSliceError, BStaticSource};
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

impl<T, S: BStaticSource<T> + BKnownEndSource> BKnownEndSource for BIntoDynamic<T, S> {
    fn b_end_offset(&self) -> Self::Offset { self.inner.b_end_offset() }
}

impl<
    T,
    E: From<BSliceError>,
    S: BStaticSource<T, Error = E> + BSliceableSource<Slice, E>,
    Slice
> BSliceableSource<Slice, E> for BIntoDynamic<T, S>  {
    fn b_full_slice(&self) -> Result<Slice, Self::Error> { self.inner.b_full_slice() }

    fn b_slice(&self, range: Range<Self::Offset>) -> Result<Slice, Self::Error> {
        self.inner.b_slice(range)
    }
}


impl<T, S: BStaticSource<T>> BDynamicSourceBase for BIntoDynamic<T, S> {
    fn b_position(&self) -> Self::Offset { self.index }
}

impl<T, S: BStaticSource<T>> BSeekableSource for BIntoDynamic<T, S> {
    fn b_jump_to(&mut self, offset: Self::Offset) -> Result<(), Self::Error> {
        Ok(self.index = offset)
    }
}


impl<T, S: BStaticSource<T>> BDynamicSource<T> for BIntoDynamic<T, S> {

    fn b_next(&mut self) -> Result<Option<Self::Token>, Self::Error> {
        let (next_position, next_token) = self.inner.b_next_at(self.index)?;
        self.b_jump_to(next_position)?;
        return Ok(next_token)
    }
}