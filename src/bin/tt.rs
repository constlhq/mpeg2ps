use mpeg2ps::ps::{PsPacketReader, ReadPsPacket};
use std::fs::File;

fn main() {
    let file = File::open("/home/lhq/work/2023/ezk/h265+g711a+ps.mpg").unwrap();
    let mut ps_packet_reader = PsPacketReader::new(file);
    for i in 0..4 {
        let xx = ps_packet_reader.read_ps_packet();

        println!("{:?}", xx.unwrap());
    }
}
