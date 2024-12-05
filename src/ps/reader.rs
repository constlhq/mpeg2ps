use crate::pes::{PesHeader, PesPacket, PesPacketReader, ReadPesPacket};
use crate::ps::payload::{Bytes, Null, Pes};
use crate::ps::psm::PsMap;
use crate::ps::system_header::PsSystemHeader;
use crate::ps::{Pid, PsHeader, PsPacket, PsPayload};
use crate::{track_io, ErrorKind, Result};
use byteorder::ReadBytesExt;
use std::collections::HashMap;
use std::io::{BufRead, Read, Seek};
use trackable::{track, track_assert_eq};

/// The `ReadTsPacket` trait allows for reading TS packets from a source.
pub trait ReadPsPacket {
    /// Reads a TS packet.
    ///
    /// If the end of the stream is reached, it will return `Ok(None)`.
    fn read_ps_packet(&mut self) -> Result<Option<PsPacket>>;
}

/// TS packet reader.
#[derive(Debug)]
pub struct PsPacketReader<R> {
    stream: R,
}
impl<R: Read> PsPacketReader<R> {
    /// Makes a new `TsPacketReader` instance.
    pub fn new(stream: R) -> Self {
        PsPacketReader { stream }
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
    fn read_ps_packet(&mut self) -> Result<Option<PsPacket>> {
        let mut ps_started = false;
        let mut peek = [0u8; 4];

        track_io!(self.stream.read_exact(&mut peek[0..3]))?;

        loop {
            // self.stream

            let ty = track_io!(self.stream.read_u8())?;

            if peek[0] == 0 && peek[1] == 0 && peek[2] == 1 {
                if ty < 0xB9 {
                    peek.rotate_left(1);
                    peek[2] = ty;
                    continue;
                }

                if !ps_started {
                    if ty != 0xBA {
                        peek.rotate_left(1);
                        peek[2] = ty;
                        continue;
                    }

                    ps_started = true;
                }

                match ty {
                    0xBA => {
                        // PS Header
                        peek[3] = ty;
                        println!("HANDLE PS HEADER");
                        let header = track!(PsHeader::read_from(peek.chain(self.stream.by_ref())))?;
                        println!("{:?}", header);

                        self.stream.read_exact(&mut peek);

                        continue;
                    }
                    0xBB => {
                        // PS system header

                        peek[3] = ty;
                        println!("HANDLE PS SYSTEM HEADER");
                        let header =
                            track!(PsSystemHeader::read_from(peek.chain(self.stream.by_ref())))?;
                        println!("parseed system header");

                        self.stream.read_exact(&mut peek[..3]);

                        println!("PEEK {:?}", peek);

                        continue;
                    }
                    0xBC => {
                        // PS msp
                        peek[3] = ty;
                        println!("HANDLE PS MAP");
                        let header = track!(PsMap::read_from(peek.chain(self.stream.by_ref())))?;
                        self.stream.read_exact(&mut peek[..3]);
                        println!("PEEK {:?}", peek);
                        continue;
                    }
                    0xB9 => {
                        peek[3] = ty;
                        println!("HANDLE PS FINISH:{:?}", peek);


                        continue;
                    }
                    _ => {
                        peek[3] = ty;
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
}

#[derive(Debug, Clone)]
enum PidKind {
    Pmt,
    Pes,
}
