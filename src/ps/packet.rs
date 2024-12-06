use crate::ps::payload::{Bytes, Null, Pes};
use crate::ps::psm::PsStreamMapExt;
use crate::ps::system_header::{PsSystemHeader, PsSystemSystemHeaderExt};
use crate::{track_io, ErrorKind, Result};
use bitfield::bitfield;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::ptr::read;
use trackable::{track, track_assert, track_assert_eq};

bitfield! {
    pub struct PsHeader(MSB0[u8]);
    impl Debug;
    u8;
    pub u32,pack_start_code,set_pack_start_code:31,0;
    pub u8,zero_one,set_zero_one:33,32;
    pub u8,system_clock_reference_base1,set_system_clock_reference_base1 :36,34;
    pub bool,marker_bit1,set_marker_bit1:37;
    pub u16,system_clock_reference_base2,set_system_clock_reference_base2: 52,38;
    pub bool, marker_bit2,set_marker_bit2:53;
    pub u16,system_clock_reference_base3,set_system_clock_reference_base3:68,54;
    pub bool,marker_bit3,set_marker_bit3:69;
    pub u16,system_clock_reference_extension,set_system_clock_reference_extension:78,70;
    pub bool,marker_bit4,set_marker_bit4:79;
    pub u32,program_mux_rate,set_program_mux_rate:101,80;
    pub bool,marker_bit5,set_marker_bit5:102;
    pub bool,marker_bit6,set_marker_bit6:103;
    pub u8,reserved,set_reserved:108,104;
    pub u8,pack_stuffing_length,set_pack_stuffing_length:111,109;
}

impl PsHeader<[u8; 14]> {
    pub fn stuffing_byte(&self) -> Option<u64> {
        let pack_stuffing_length = self.pack_stuffing_length();
        if pack_stuffing_length == 0 {
            None
        } else {
            let max_stuffing_bytes: u64 = u64::MAX >> 8;
            Some(max_stuffing_bytes >> pack_stuffing_length)
        }
    }
    pub(super) fn read_from<R: Read>(mut reader: R) -> Result<Self> {
        let mut pack_header_buf = [0u8; 14];
        track_io!(reader.read_exact(&mut pack_header_buf))?;
        let ps_header = PsHeader(pack_header_buf);

        match ps_header.pack_stuffing_length() {
            0 => {}
            1 => {
                let _ = reader.read_u8();
            }
            2 => {
                let _ = reader.read_u16::<BigEndian>();
            }
            3 => {
                let _ = reader.read_u24::<BigEndian>();
            }
            4 => {
                let _ = reader.read_u32::<BigEndian>();
            }
            5 => {
                let _ = reader.read_u8();
                let _ = reader.read_u32::<BigEndian>();
            }
            6 => {
                let _ = reader.read_u48::<BigEndian>();
            }
            7 => {
                let _ = reader.read_u8();
                let _ = reader.read_u48::<BigEndian>();
            }
            _ => {
                unreachable!()
            }
        };

        Ok(ps_header)
    }
}

#[allow(missing_docs, clippy::large_enum_variant)]
#[derive(Debug)]
pub enum PsPack {
    PsHeader(PsHeader<[u8; 14]>),
    PsStreamMapExt(PsStreamMapExt),
    PsFinish(u32),
    PsSystemHeaderExt(PsSystemSystemHeaderExt),
    Pes(Pes),
}

/// TS packet payload.
#[allow(missing_docs, clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PsPayload {
    Pes(Pes),
    Null(Null),
    Raw(Bytes),
}
impl PsPayload {
    fn write_to<W: Write>(&self, writer: W) -> Result<()> {
        match *self {
            PsPayload::Pes(ref x) => track!(x.write_to(writer)),
            PsPayload::Null(_) => Ok(()),
            PsPayload::Raw(ref x) => track!(x.write_to(writer)),
        }
    }
}
