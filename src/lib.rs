pub mod error;
pub mod es;
pub mod pes;
pub mod time;
pub mod ps;

use error::Error;
use error::ErrorKind;

/// This crate specific `Result` type.
pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! track_io {
    ($expr:expr) => {
        $expr.map_err(|e: ::std::io::Error| {
            use trackable::error::ErrorKindExt;
            track!(crate::Error::from(crate::ErrorKind::Other.cause(e)))
        })
    };
}
