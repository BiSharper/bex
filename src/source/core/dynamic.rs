use crate::source::{BIntoPeekable, BPeekableSource, BSeekableSource, BSourceBase, BSourceMeta};

pub trait BDynamicSourceBase: BSourceMeta + BSeekableSource {
    fn b_position(&self) -> Self::Offset;

    //some sources might have a cache so the underlying position is not the actual amount of bytes
    //that have been read, so we leave this overflow in said case to be overridden
    fn b_traversed(&self) -> Self::Offset { self.b_position() }

}

pub trait BDynamicSource<T>: BSourceBase<T> + BDynamicSourceBase {
    fn b_next(&mut self) -> Result<Option<Self::Token>, Self::Error>;

    fn into_peekable(self) -> Option<impl BPeekableSource<T>> where
        Self: Sized,
        Self::Token: Clone,
        T: Clone
    {
        Some(BIntoPeekable::create(self, None))
    }
}