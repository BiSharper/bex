use std::ops::{Deref, DerefMut, Range};
use crate::BErrorScoped;
use crate::source::base::{BKnownEndSource, BKnownStartSource, BOwnedStrSource, BSliceableSource, BSourceBase, BSourceMeta, BStrSource};
use crate::source::{BSimpleError, BSliceError, BStaticSource, BStringError};

type Slice<'a, T> = &'a [T];

pub struct BSlice<'a, T>(Slice<'a, T>);
pub struct BByteSlice<'a>(Slice<'a, u8>);

impl<'a, T> BErrorScoped for Slice<'a, T> {
    type Error = BSliceError;
}

impl<'a> BErrorScoped for BByteSlice<'a> {
    type Error = BSimpleError;
}

impl<'a> BSourceMeta for BByteSlice<'a> {
    type Offset = usize;
}

impl<'a, T> From<&'a [T]> for BSlice<'a, T> {
    fn from(slice: &'a [T]) -> Self { BSlice(slice) }
}

impl<'a> From<&'a [u8]> for BByteSlice<'a> {
    fn from(slice: &'a [u8]) -> Self { BByteSlice(slice) }
}

impl<'a> Deref for BByteSlice<'a> {
    type Target = &'a [u8];

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<'a> DerefMut for BByteSlice<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<'a, T> Deref for BSlice<'a, T> {
    type Target = &'a [T];

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<'a, T> DerefMut for BSlice<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<'a, T> BSourceMeta for Slice<'a, T> {
    type Offset = usize;
}

impl<'a, T> BSourceBase<T> for Slice<'a, T> {
    type Token = &'a T;
}

impl<'a, T> BKnownStartSource for Slice<'a, T> {
    fn b_start(&self) -> Self::Offset { 0 }
}

impl<'a, T> BKnownEndSource for Slice<'a, T> {
    fn b_end_offset(&self) -> Self::Offset { self.len() }
}

impl<'a, T> BSliceableSource<&'a [T], BSliceError> for Slice<'a, T> {
    fn b_full_slice(&self) -> Result<&'a [T], Self::Error> { Ok(&self) }

    fn b_slice(&self, range: Range<Self::Offset>) -> Result<&'a [T], Self::Error> {
        Ok(&self[range])
    }
}

impl<'a, T> BStaticSource<T> for Slice<'a, T> {
    fn b_next_at(&mut self, offset: Self::Offset) -> Result<(Self::Offset, Option<Self::Token>), Self::Error> {
        if let Some(tok) = self.get(offset) {
            return Ok((offset + 1, Some(tok)))
        }
        Ok((offset, None))
    }
}

impl<'a> BSliceableSource<&'a str, BSimpleError> for BByteSlice<'a> {
    fn b_full_slice(&self) -> Result<&'a str, Self::Error> {
        std::str::from_utf8(self.0.b_full_slice()?).map_err(|err| BSimpleError::String(BStringError::RustUTF(err)))
    }

    fn b_slice(&self, range: Range<Self::Offset>) -> Result<&'a str, Self::Error> {
        std::str::from_utf8(self.0.b_slice(range)?).map_err(|err| BSimpleError::String(BStringError::RustUTF(err)))
    }
}
impl<'a> BSliceableSource<&'a [u8], BSimpleError> for BByteSlice<'a> {
    fn b_full_slice(&self) -> Result<&'a [u8], Self::Error> {
        self.0.b_full_slice().map_err(BSimpleError::Slice)
    }

    fn b_slice(&self, range: Range<Self::Offset>) -> Result<&'a [u8], Self::Error> {
        self.0.b_slice(range).map_err(BSimpleError::Slice)
    }
}
impl<'a> BKnownStartSource for BByteSlice<'a> {
    fn b_start(&self) -> Self::Offset { self.0.b_start() }
}

impl<'a> BSourceBase<u8> for BByteSlice<'a> {
    type Token = &'a u8;
}

impl<'a> BStaticSource<u8> for BByteSlice<'a> {
    fn b_next_at(&mut self, offset: Self::Offset) -> Result<(Self::Offset, Option<Self::Token>), Self::Error> {
        self.0.b_next_at(offset).map_err(BSimpleError::Slice)
    }
}

impl<'a> BSourceBase<char> for BByteSlice<'a> {
    type Token = char;
}

impl<'a> BStrSource<BSimpleError> for BByteSlice<'a> {

}

impl<'a> BOwnedStrSource<'a, BSimpleError> for BByteSlice<'a> {

}

impl<'a> BStaticSource<char> for BByteSlice<'a> {
    fn b_next_at(&mut self, offset: Self::Offset) -> Result<(Self::Offset, Option<Self::Token>), Self::Error> {
        let source_length = self.0.len();
        if offset >= source_length {
            return Ok((offset, None))
        }

        let weight = match self.0[offset] {
            x if x < 128 => 1,
            x if x < 224 => 2,
            x if x < 240 => 3,
            x if x < 248 => 4,
            _ => return Err(BSimpleError::from(BStringError::InvalidUTF))
        };

        let end = offset + weight;

        if end > source_length {
            return Err(BSimpleError::from(BStringError::IncompleteUTF))
        }

        let mut decoded = std::str::from_utf8(&self.0[offset..end])
            .map_err(|err| BSimpleError::String(BStringError::RustUTF(err)))?.chars();

        match decoded.next(){
            None => Ok((offset, None)),
            Some(char) => Ok((offset + char.len_utf8(), Some(char)))
        }
    }
}

