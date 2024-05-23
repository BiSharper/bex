use std::io;
use std::io::Read;
//todo
//
// pub trait BReadable<R: Read, E: From<io::Error>> {
//     fn try_read(reader: &mut R) -> Result<Option<Self>, E>;
// }
//
// pub struct BReadSource<
//     R: Read,
//     E: From<io::Error>,
//     T: BReadable<R, E>
// > {
//     bytes_read: usize,
//     inner: R
// }