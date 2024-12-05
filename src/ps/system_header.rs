use crate::Result;
use std::io::Read;
use trackable::track;
#[derive(Default,Debug,PartialEq,Eq)]
pub struct PsSystemHeader {
    pub start_code: u32, //0x000001BB
    pub header_length: u16,
    pub rate_bound: u32, //24b
    pub csps_flag: bool,
    pub fixed_flag: bool,
    pub audio_bound: u8, //6b
    pub video_bound: u8, //5b
    pub marker_bit: bool,
    pub system_video_lock_flag: bool,
    pub system_audio_lock_flag: bool,
    pub reserved_byte: u8,
}

impl PsSystemHeader {
    pub(super) fn read_from<R: Read>(mut reader: R) -> Result<Self> {
        // #define BUF2U16(buf) (((buf)[0] << 8) | (buf)[1])

        let mut peek = [0; 12];
        reader.read_exact(&mut peek);
        let psize = 6 + ((peek[4] as u16) << 8 | peek[5] as u16) as usize;

        println!("psize: {}", psize);

        // let mut temp = [0; 12];
        // reader.read_exact(&mut temp);

        // FIXME
        Ok(Self::default())
    }
}
