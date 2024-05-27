use std::str::Utf8Error;

pub enum BSliceError {
    InvalidRange,
}

pub enum BStringError {
    RustUTF(Utf8Error),
    InvalidUTF,
    IncompleteUTF
}
pub enum BSimpleError {
    String(BStringError),
    Slice(BSliceError)
}

impl From<Utf8Error> for BStringError {
    fn from(value: Utf8Error) -> Self { Self::RustUTF(value) }
}

impl From<BSliceError> for BSimpleError {
    fn from(value: BSliceError) -> Self { Self::Slice(value) }
}

impl From<BStringError> for BSimpleError {
    fn from(value: BStringError) -> Self { Self::String(value) }
}
