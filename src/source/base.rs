use std::borrow::Borrow;
use std::hash::Hash;
use std::ops::{Add, Range, Sub};
use crate::{BErrorScoped, source::{BSliceError}};
use crate::source::BStringError;

pub trait BSourceMeta: BErrorScoped {
    type Offset:
        Copy + Hash + Ord +
        Into<usize> +
        Sub<Self::Offset, Output = Self::Offset> +
        Add<Self::Offset, Output = Self::Offset>;
}

pub trait BSeekableSource: BSourceMeta {
    fn b_jump_to(&mut self, offset: Self::Offset) -> Result<(), Self::Error>;
}

pub trait BKnownEndSource: BSourceMeta {
    fn b_end_offset(&self) -> Self::Offset;
}

pub trait BKnownStartSource: BSourceMeta {
    fn b_start(&self) -> Self::Offset;

    //some sources like a buffered source for ex might want to use start for something like the
    //start of the current buffer, so we leave this overflow in said case to be overridden
    fn b_actual_start(&self) -> Self::Offset { self.b_start() }
}

pub trait BSliceableSource<S, E: From<BSliceError>>: BSourceMeta<Error = E> {
    fn b_full_slice(&self) -> Result<S, Self::Error>;

    fn b_slice(&self, range: Range<Self::Offset>) -> Result<S, Self::Error>;
}

pub trait BSourceBase<T>: BErrorScoped + BKnownStartSource {
    type Token: Borrow<T>;
}

pub trait BStrSource<E: From<BStringError>>:
    BSourceBase<char, Error = E> +
    BSourceBase<u8, Error = E> {
}

pub trait BOwnedStrSource<'a, E: From<BStringError> + From<BSliceError>>:
    BStrSource<E> +
    BSliceableSource<&'a str, E> +
    BSliceableSource<&'a [u8], E> {
}

