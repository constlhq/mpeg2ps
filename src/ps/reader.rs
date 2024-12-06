use crate::pes::{PesHeader, PesPacket, PesPacketReader, ReadPesPacket};
use crate::ps::packet::{PsHeader, PsPack};
use crate::ps::payload::{Bytes, Null, Pes};
use crate::ps::psm::{ PsStreamMapExt};
use crate::ps::system_header::{PsSystemHeader, PsSystemSystemHeaderExt};
use crate::ps::{Pid, PsPayload};
use crate::{track_io, ErrorKind, Result};
use byteorder::ReadBytesExt;
use std::collections::HashMap;
use std::io::{BufRead, Read, Seek};
use trackable::{track, track_assert_eq};

/// The `ReadTsPacket` trait allows for reading TS packets from a source.
pub trait ReadPsPacket {
    /// Reads a TS packet.
    /// If the end of the stream is reached, it will return `Ok(None)`.
    fn read_ps_packet(&mut self) -> Result<Option<PsPack>>;
    fn get_ps_pack_started(&self) -> bool;
    fn set_ps_pack_started(&mut self, state: bool);
}

/// TS packet reader.
#[derive(Debug)]
pub struct PsPacketReader<R> {
    stream: R,
    ps_pack_started: bool,
}
impl<R: Read> PsPacketReader<R> {
    /// Makes a new `TsPacketReader` instance.
    pub fn new(stream: R) -> Self {
        PsPacketReader {
            stream,
            ps_pack_started: false,
        }
    }

    /// Returns a reference to the underlaying byte stream.
    pub fn stream(&self) -> &R {
        &self.stream
    }

    /// Converts `TsPacketReader` into the underlaying byte stream `R`.
    pub fn into_stream(self) -> R {
        self.stream
    }
}
impl<R: Read> ReadPsPacket for PsPacketReader<R> {
    fn read_ps_packet(&mut self) -> Result<Option<PsPack>> {
        let mut peek = [0u8; 4];
        track_io!(self.stream.read_exact(&mut peek[0..3]))?;
        loop {
            let ty = track_io!(self.stream.read_u8())?;
            if peek[0] == 0 && peek[1] == 0 && peek[2] == 1 {
                if ty < 0xB9 {
                    peek.rotate_left(1);
                    peek[2] = ty;
                    continue;
                }

                if !self.get_ps_pack_started() {
                    if ty != 0xBA {
                        peek.rotate_left(1);
                        peek[2] = ty;
                        continue;
                    }

                    self.ps_pack_started = true;
                }

                peek[3] = ty;
                match ty {
                    0xBA => {
                        // PS Header
                        println!("HANDLE PS HEADER");
                        let header = track!(PsHeader::read_from(peek.chain(self.stream.by_ref())))?;
                        println!("{:?}", header);
                        return Ok(Some(PsPack::PsHeader(header)));
                    }
                    0xBB => {
                        // PS system header
                        println!("HANDLE PS SYSTEM HEADER");
                        let header = track!(PsSystemSystemHeaderExt::read_from(
                            peek.chain(self.stream.by_ref())
                        ))?;

                        return Ok(Some(PsPack::PsSystemHeaderExt(header)));
                    }
                    0xBC => {
                        // PS msp
                        println!("HANDLE PS MAP");
                        let header =
                            track!(PsStreamMapExt::read_from(peek.chain(self.stream.by_ref())))?;
                        return Ok(Some(PsPack::PsStreamMapExt(header)));
                    }
                    0xB9 => {
                        println!("HANDLE PS FINISH:{:?}", peek);

                        self.ps_pack_started = false;

                        return Ok(Some(PsPack::PsFinish(0x000001B9)));
                    }
                    _ => {
                        println!("HANDLE PES HEADER");
                        // let mut cc = Vec::from(&peek);
                        // cc.push(x);
                        //
                        // println!("{:08X?}", cc);

                        // let mut rrr = PesPacketReader::new(self);

                        // let pes = rrr.read_pes_packet()?;

                        let pes = Pes::read_from(peek.chain(self.stream.by_ref()));

                        // let pesh = track!(PesHeader::read_from(peek.chain(self.stream.by_ref())))?;

                        println!("{:?}", pes);

                        continue;
                    }
                }
            } else {
                peek.rotate_left(1);
                peek[2] = ty;
                continue;
            }
        }
    }

    fn get_ps_pack_started(&self) -> bool {
        self.ps_pack_started
    }

    fn set_ps_pack_started(&mut self, state: bool) {
        self.ps_pack_started = state;
    }
}

#[derive(Debug, Clone)]
enum PidKind {
    Pmt,
    Pes,
}
