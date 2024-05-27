use crate::source::{BDynamicSource, BIntoDynamic, BKnownStartSource, BSourceBase, BSourceMeta};

pub trait BStaticSourceBase: BSourceMeta + BKnownStartSource {

}

pub trait BStaticSource<T>: BSourceBase<T> + BStaticSourceBase {
    fn b_next_at(&mut self, offset: Self::Offset) -> Result<(Self::Offset, Option<Self::Token>), Self::Error>;

    fn into_dynamic(self, index: Self::Offset) -> Option<impl BDynamicSource<T>> where Self: Sized {
        Some(BIntoDynamic::create(self, index))
    }
}
