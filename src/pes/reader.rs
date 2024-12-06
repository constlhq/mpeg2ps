use crate::pes::PesPacket;
use crate::ps::payload::{Bytes, Pes};
use crate::ps::{Pid, PsPayload, ReadPsPacket};
use crate::{ErrorKind, Result};
use std::collections::HashMap;
use trackable::{track, track_assert, track_assert_some};

/// The `ReadPesPacket` trait allows for reading PES packets from a source.
pub trait ReadPesPacket {
    /// Reads a PES packet.
    ///
    /// If the end of the stream is reached, it will return `Ok(None)`.
    fn read_pes_packet(&mut self) -> Result<Option<PesPacket<Vec<u8>>>>;
}

/// PES packet reader.
#[derive(Debug)]
pub struct PesPacketReader<R> {
    ps_packet_reader: R,
    pes_packets: Option<PartialPesPacket>,
    eos: bool,
}
impl<R: ReadPsPacket> PesPacketReader<R> {
    /// Makes a new `PesPacketReader` instance.
    pub fn new(ps_packet_reader: R) -> Self {
        PesPacketReader {
            ps_packet_reader,
            pes_packets: None,
            eos: false,
        }
    }

    /// Returns a reference to the underlaying TS packet reader.
    pub fn ps_packet_reader(&self) -> &R {
        &self.ps_packet_reader
    }

    /// Converts `PesPacketReader` into the underlaying TS packet reader.
    pub fn into_ps_packet_reader(self) -> R {
        self.ps_packet_reader
    }

    fn handle_eos(&mut self) -> Result<Option<PesPacket<Vec<u8>>>> {
        if let Some(key) = self.pes_packets.as_ref() {
            let partial = self.pes_packets.take().unwrap();
            track_assert!(
                partial.data_len.is_none() || partial.data_len == Some(partial.packet.data.len()),
                ErrorKind::InvalidInput,
                "Unexpected EOS"
            );
            Ok(Some(partial.packet))
        } else {
            Ok(None)
        }
    }

    fn handle_pes_payload(&mut self, pes: Pes) -> Result<Option<PesPacket<Vec<u8>>>> {
        let data_len = if pes.pes_packet_len == 0 {
            None
        } else {
            let optional_header_len = pes.header.optional_header_len();
            track_assert!(
                pes.pes_packet_len >= optional_header_len,
                ErrorKind::InvalidInput,
                "pes.pes_packet_len={}, optional_header_len={}",
                pes.pes_packet_len,
                optional_header_len
            );
            Some((pes.pes_packet_len - optional_header_len) as usize)
        };

        let mut data = Vec::with_capacity(data_len.unwrap_or(pes.data.len()));
        data.extend_from_slice(&pes.data);

        let packet = PesPacket {
            header: pes.header,
            data,
        };
        let partial = PartialPesPacket { packet, data_len };
        if let Some(pred) = self.pes_packets.replace(partial) {
            track_assert!(
                pred.data_len.is_none() || pred.data_len == Some(pred.packet.data.len()),
                ErrorKind::InvalidInput,
                "Mismatched PES packet data length: actual={}, expected={}",
                pred.data_len.expect("Never fails"),
                pred.packet.data.len()
            );
            Ok(Some(pred.packet))
        } else {
            Ok(None)
        }
    }

    fn handle_raw_payload(&mut self, data: &Bytes) -> Result<Option<PesPacket<Vec<u8>>>> {
        let mut partial = track_assert_some!(self.pes_packets.take(), ErrorKind::InvalidInput);
        partial.packet.data.extend_from_slice(data);
        if Some(partial.packet.data.len()) == partial.data_len {
            Ok(Some(partial.packet))
        } else {
            if let Some(expected) = partial.data_len {
                track_assert!(
                    partial.packet.data.len() <= expected,
                    ErrorKind::InvalidInput,
                    "Too large PES packet data: actual={}, expected={}",
                    partial.packet.data.len(),
                    expected
                );
            }
            self.pes_packets.replace(partial);
            Ok(None)
        }
    }
}
impl<R: ReadPsPacket> ReadPesPacket for PesPacketReader<R> {
    fn read_pes_packet(&mut self) -> Result<Option<PesPacket<Vec<u8>>>> {
        if self.eos {
            return track!(self.handle_eos());
        }

        // while let Some(ts_packet) = track!(self.ps_packet_reader.read_ps_packet())? {
        //     let result = match ts_packet.payload {
        //         Some(PsPayload::Pes(payload)) => track!(self.handle_pes_payload(payload))?,
        //         Some(PsPayload::Raw(payload)) => track!(self.handle_raw_payload(&payload))?,
        //         _ => None,
        //     };
        //     if result.is_some() {
        //         return Ok(result);
        //     }
        // }

        self.eos = true;
        track!(self.handle_eos())
    }
}

#[derive(Debug)]
struct PartialPesPacket {
    packet: PesPacket<Vec<u8>>,
    data_len: Option<usize>,
}
