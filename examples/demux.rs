use mpeg2ps::ps::packet::PsPack;
use mpeg2ps::ps::{PsPacketReader, ReadPsPacket};
use std::fs::File;

fn main() {
    let file = File::open("examples/test.h264.ps").unwrap();
    let mut ps_packet_reader = PsPacketReader::new(file);
    loop {
        let packet = ps_packet_reader.read_ps_packet();

        if packet.is_err() {
            break;
        } else {
            match packet.unwrap() {
                None => {}
                Some(ps_pack) => match ps_pack {
                    PsPack::PsHeader(ps_header) => {
                        println!("{:?}", ps_header);
                    }
                    PsPack::PsStreamMapExt(stream_map_ext) => {
                        println!("{:?}", stream_map_ext);
                    }
                    PsPack::PsFinish(end_code) => {
                        println!("{:0X}", end_code);
                    }
                    PsPack::PsSystemHeaderExt(system_header) => {
                        println!("{:?}", system_header);
                    }
                    PsPack::Pes(pes) => {
                        println!("{:?}", pes);
                    }
                },
            }
        }
    }
}
