use std::ops::Range;
use crate::BErrorScoped;
use crate::source::base::{BKnownEndSource, BKnownStartSource, BOwnedStrSource, BSliceableSource, BSourceBase, BSourceMeta, BStrSource};
use crate::source::{BSimpleError, BStaticSource};

impl<'a> BErrorScoped for &'a str {
    type Error = BSimpleError;
}

impl<'a> BSourceMeta for &'a str {
    type Offset = usize;
}

impl<'a> BSourceBase<char> for &'a str {
    type Token = char;
}

impl<'a> BSourceBase<u8> for &'a str {
    type Token = u8;
}

impl<'a> BKnownStartSource for &'a str {
    fn b_start(&self) -> Self::Offset { 0 }
}

impl<'a> BKnownEndSource for &'a str {
    fn b_end_offset(&self) -> Self::Offset { self.len() }
}

impl<'a> BSliceableSource<&'a str, BSimpleError> for &'a str {
    fn b_full_slice(&self) -> Result<&'a str, Self::Error> { Ok(&self) }

    fn b_slice(&self, range: Range<Self::Offset>) -> Result<&'a str, Self::Error> {
        Ok(&self[range])
    }
}

impl<'a> BSliceableSource<&'a [u8], BSimpleError> for &'a str {
    fn b_full_slice(&self) -> Result<&'a [u8], Self::Error> { Ok(self.as_bytes()) }

    fn b_slice(&self, range: Range<Self::Offset>) -> Result<&'a [u8], Self::Error> {
        Ok(&self.as_bytes()[range])
    }
}

impl<'a> BStaticSource<char> for &'a str {
    fn b_next_at(&mut self, offset: Self::Offset) -> Result<(Self::Offset, Option<Self::Token>), Self::Error> {
        if offset < self.len() {
            let c = unsafe {
                self.get_unchecked(offset..)
                    .chars()
                    .next()
                    .unwrap_unchecked()
            };
            return Ok((offset + c.len_utf8(), Some(c)))
        }
        return Ok((offset, None))
    }
}

impl<'a> BStaticSource<u8> for &'a str {
    fn b_next_at(&mut self, offset: Self::Offset) -> Result<(Self::Offset, Option<Self::Token>), Self::Error> {
        Ok((offset + 1, self.as_bytes().get(offset).copied()))
    }
}

impl<'a> BStrSource<BSimpleError> for &'a str {

}

impl<'a> BOwnedStrSource<'a, BSimpleError> for &'a str {

}