use std::io::Read;
use crate::Result;

#[derive(Debug,Default)]
pub struct PsMap {
    pub start_code: u32,
    pub header_length: u16,
    pub ps_map_version: u8,
    pub reserved1: u8,
    pub current_next_indicator: bool,
    pub marker_bit: bool,
    pub reserved2: u8, // 7b
    pub ps_info_length: u16,
    pub es_map_length: u16,
}

impl PsMap {
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

pub struct PsMapEs {
    pub stream_type: u8,
    pub es_id: u8,
    pub es_info_length: u16,
}
