mod crc;
pub mod error;
pub mod es;
pub mod pes;
pub mod ps;
pub mod time;
mod util;
use trackable::track;

use error::Error;
use error::ErrorKind;

/// This crate specific `Result` type.
pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! track_io {
    ($expr:expr) => {
        $expr.map_err(|e: ::std::io::Error| {
            use trackable::error::ErrorKindExt;
            trackable::track!(crate::Error::from(crate::ErrorKind::Other.cause(e)))
        })
    };
}

#[cfg(test)]
mod tests {
    use crate::pes::PesPacketReader;
    use crate::ps::{PsPacketReader, ReadPsPacket};
    use std::fs::File;

    #[test]
    fn it_works() {
        let file = File::open("/home/lhq/work/2023/ezk/ps_h264_g711.ps").unwrap();
        let mut ps_packet_reader = PsPacketReader::new(file);

        let xx = ps_packet_reader.read_ps_packet();

        assert!(xx.is_ok());
    }
}
