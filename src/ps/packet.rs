use crate::ps::payload::{Bytes, Null, Pes};
use crate::ps::psm::PsMap;
use crate::ps::system_header::PsSystemHeader;
use crate::{track_io, ErrorKind, Result};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::ptr::read;
use trackable::{track, track_assert, track_assert_eq};

/// Transport stream packet.
#[allow(missing_docs)]
#[derive(Debug)]
pub struct PsPacket {
    pub header: PsHeader,
    pub system_header: Option<PsSystemHeader>,
    pub psm: Option<PsMap>,
}
impl PsPacket {
    /// Size of a packet in bytes.
    pub const SIZE: usize = 14 * 8;

    /// Synchronization byte.
    ///
    /// Each packet starts with this byte.
    pub const SYNC_BYTE: u8 = 0x47;

    pub const START_CODE: u32 = 0x000001BA;
    pub(super) fn write_to<W: Write>(&self, mut writer: W) -> Result<()> {

        unimplemented!()
        // let mut payload_buf = [0; PsPacket::SIZE - 4];
        // let payload_len = if let Some(ref payload) = self.payload {
        //     let mut writer = Cursor::new(&mut payload_buf[..]);
        //     track!(payload.write_to(&mut writer))?;
        //     writer.position() as usize
        // } else {
        //     0
        // };
        //
        // let free_len = PsPacket::SIZE - 4 - payload_len;
        //
        // let payload_unit_start_indicator = !matches!(
        //     self.payload,
        //     Some(PsPayload::Raw(_)) | Some(PsPayload::Null(_)) | None
        // );
        //
        // track_io!(writer.write_all(&payload_buf[..payload_len]))?;
        // Ok(())
    }
}

/// TS packet header.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PsHeader {
    pub start_code: u32,            //32b
    pub scr: [u8; 6],               // 48b
    pub program_mux_rate: [u8; 3],  // 24b
    pub packet_stuffing_length: u8, //8b
}
impl PsHeader {
    pub(super) fn read_from<R: Read>(mut reader: R) -> Result<Self> {
        let start_code = track_io!(reader.read_u32::<BigEndian>())?;

        println!("{:08X}", start_code);
        track_assert_eq!(start_code, PsPacket::START_CODE, ErrorKind::InvalidInput);
        let mut scr = [0; 6];
        track_io!(reader.read_exact(&mut scr))?;
        let mut program_mux_rate = [0; 3];
        track_io!(reader.read_exact(&mut program_mux_rate))?;
        let packet_stuffing_length = track_io!(reader.read_u8())? & 0x07;
        let header = PsHeader {
            start_code,
            scr,
            program_mux_rate,
            packet_stuffing_length,
        };
        Ok(header)
    }

    fn write_to<W: Write>(&self, mut writer: W) -> Result<()> {
        track_io!(writer.write_u32::<BigEndian>(PsPacket::START_CODE))?;
        track_io!(writer.write(&self.scr))?;
        track_io!(writer.write(&self.program_mux_rate))?;
        track_io!(writer.write_u8(self.packet_stuffing_length))?;

        Ok(())
    }
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
