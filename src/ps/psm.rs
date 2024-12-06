use crate::crc::Crc32;
use crate::es::StreamId;
use crate::{track_io, Result};
use bitfield::bitfield;
use byteorder::{BigEndian, ReadBytesExt};
use std::fmt::Debug;
use std::io::Read;
use std::ptr::read;
use trackable::track;

bitfield! {
    pub struct PsStreamMap(MSB0 [u8]);
    impl Debug;
    pub u32,packet_start_code_prefix,set_packet_start_code_prefix:23,0;
pub u8,map_stream_id,set_map_stream_id:31,24;
pub u16,program_stream_map_length,set_program_stream_map_length:47,32;
pub bool,current_next_indicator,set_current_next_indicator:48;
pub u8,reserved1,set_reserved1:50,49;
pub u8,program_stream_map_version,set_program_stream_map_version:55,51;
pub u8,reserved2,set_reserved2:62,56;
pub bool,marker_bit,set_marker_bit:63;
pub u16,program_stream_info_length,set_program_stream_info_length:79,64;
}

impl PsStreamMap<[u8; 10]> {
    pub(super) fn read_from<R: Read>(mut reader: R) -> Result<Self> {
        let mut buf = [0; 10];

        reader.read_exact(&mut buf);

        let ps_stream_map = PsStreamMap(buf);

        Ok(ps_stream_map)
    }
}

pub struct ElementaryStreamMap {
    pub stream_type: u8,
    pub elementary_stream_id: u8,
    pub elementary_stream_info_length: u16,
    pub elementary_stream_info: Option<Vec<u8>>,
}

impl Debug for ElementaryStreamMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"ElementaryStreamMap{{stream_type:0X{:02X},elementary_stream_id:0x{:02X},elementary_stream_info_length:{},elementary_stream_info:{:?}}}",self.stream_type,self.elementary_stream_id,self.elementary_stream_info_length,self.elementary_stream_info)
    }
}

impl ElementaryStreamMap {
    pub(super) fn read_from<R: Read>(mut reader: R) -> Result<Self> {
        let stream_type = track_io!(reader.read_u8())?;
        let elementary_stream_id = track_io!(reader.read_u8())?;
        let elementary_stream_info_length = track_io!(reader.read_u16::<BigEndian>())?;

        if elementary_stream_info_length > 0 {
            let mut elementary_stream_info = vec![0u8; elementary_stream_info_length as usize];
            track_io!(reader.read_exact(&mut elementary_stream_info))?;

            Ok(ElementaryStreamMap {
                stream_type,
                elementary_stream_id,
                elementary_stream_info_length,
                elementary_stream_info: Some(elementary_stream_info),
            })
        } else {
            Ok(ElementaryStreamMap {
                stream_type,
                elementary_stream_id,
                elementary_stream_info_length,
                elementary_stream_info: None,
            })
        }
    }
}

#[derive(Debug)]
pub struct PsStreamMapExt {
    pub ps_stream_map: PsStreamMap<[u8; 10]>,
    pub program_stream_info: Option<Vec<u8>>,
    pub elementary_stream_map_length: u16,
    pub elementary_stream_map: Option<Vec<ElementaryStreamMap>>,
    pub crc32: Crc32,
}

impl PsStreamMapExt {
    pub fn read_from<R: Read>(mut reader: R) -> Result<Self> {
        let ps_stream_map = PsStreamMap::read_from(reader.by_ref())?;

        let program_stream_info_length = ps_stream_map.program_stream_info_length();

        let program_stream_info = if program_stream_info_length > 0 {
            let mut program_stream_info = vec![0u8; program_stream_info_length as usize];
            track_io!(reader.by_ref().read_exact(&mut program_stream_info))?;
            Some(program_stream_info)
        } else {
            None
        };

        let elementary_stream_map_length = track_io!(reader.by_ref().read_u16::<BigEndian>())?;

        let elementary_stream_map = if elementary_stream_map_length == 0 {
            None
        } else {
            let mut elementary_stream_map: Vec<ElementaryStreamMap> = Vec::new();

            let mut accumate_length = 0usize;

            while accumate_length < elementary_stream_map_length as usize {
                let reader_ref = reader.by_ref();
                let item = ElementaryStreamMap::read_from(reader_ref)?;
                elementary_stream_map.push(item);
                accumate_length += std::mem::size_of::<ElementaryStreamMap>();
            }
            Some(elementary_stream_map)
        };

        let crc32 = track_io!(reader.read_u32::<BigEndian>())?;

        Ok(Self {
            ps_stream_map,
            program_stream_info,
            elementary_stream_map_length,
            elementary_stream_map,
            crc32: Crc32::from_u32(crc32),
        })
    }
}
