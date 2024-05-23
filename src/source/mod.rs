mod base; pub use base::*;
mod errors; pub use errors::*;
mod impls; pub use impls::*;
mod util; pub use util::*;


pub trait BStaticSource<T>: BSourceBase<T> + BSourceMeta {
    fn b_next_at(&mut self, offset: Self::Offset) -> Result<(Self::Offset, Option<Self::Token>), Self::Error>;
}

pub trait BDynamicSource<T>: BSourceBase<T> + BSourceMeta {
    fn b_position(&self) -> Self::Offset;

    fn b_seek(&mut self, offset: Self::Offset) -> Result<(), Self::Error>;

    //some sources might have a cache so the underlying position is not the actual amount of bytes
    //that have been read, so we leave this overflow in said case to be overridden
    fn b_traversed(&self) -> Self::Offset { self.b_position() }

    fn b_next(&mut self) -> Result<Option<Self::Token>, Self::Error>;
}