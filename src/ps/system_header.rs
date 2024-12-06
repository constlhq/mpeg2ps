use crate::ps::stream_table::PsSystemHeaderStreamTable;
use crate::Result;
use bitfield::bitfield;
use std::io::{repeat, Read};
use std::iter;

#[derive(Debug)]
pub struct PsSystemSystemHeaderExt {
    pub ps_system_header: PsSystemHeader<[u8; 12]>,
    pub ps_system_table: Option<Vec<PsSystemHeaderStreamTable<[u8; 3]>>>,
}

impl PsSystemSystemHeaderExt {
    pub(super) fn read_from<R: Read>(mut reader: R) -> Result<Self> {
        let reader_ref = reader.by_ref();
        let ps_system_header = PsSystemHeader::read_from(reader_ref)?;
        let st_size = 6 + ps_system_header.header_length() - 12;
        if st_size > 0 {
            let table_item_count = st_size / 3;
            assert_eq!(st_size % 3, 0);
            let mut ps_system_table = Vec::with_capacity(table_item_count as usize);
            for _ in 0..table_item_count {
                let reader_ref = reader.by_ref();
                let ps_system_table_row = PsSystemHeaderStreamTable::read_from(reader_ref)?;
                ps_system_table.push(ps_system_table_row);
            }

            Ok(PsSystemSystemHeaderExt {
                ps_system_header,
                ps_system_table: Some(ps_system_table),
            })
        } else {
            Ok(PsSystemSystemHeaderExt {
                ps_system_header,
                ps_system_table: None,
            })
        }
    }
}

bitfield! {
    pub struct PsSystemHeader(MSB0 [u8]);
    impl Debug;
    u8;
    pub u32,start_code,set_start_code:31,0; //32
    pub u16,header_length,set_header_length:47,32; //16
    pub bool,marker_bit1,set_marker_bit1:48; //1
    pub u32,rate_bound,set_rate_bound:70,49; //22
    pub bool,marker_bit2,set_marker_bit2: 71; //1
    pub u8,audio_bound,set_audio_bound:77,72; //6
    pub bool,fixed_flag,set_fixed_flag:78; //1
    pub bool,csps_flag,set_csps_flag:79;//1
    pub bool,system_audio_lock_flag,set_system_audio_lock_flag:80;//1
    pub bool,system_video_lock_flag,set_system_video_lock_flag: 81;//1
    pub bool,marker_bit3,set_marker_bit3:82;//1
    pub u8,video_bound,set_video_bound:87,83; //5
    pub bool,packet_rate_restriction_flag,set_packet_rate_restriction_flag:88; //1
    pub u8,reserved_bits,set_reserved_bits:95,89; //7

}

impl PsSystemHeader<[u8; 12]> {
    pub(super) fn read_from<R: Read>(mut reader: R) -> Result<Self> {
        let mut buf = [0; 12];
        reader.read_exact(&mut buf);
        let system_header = PsSystemHeader(buf);
        Ok(system_header)
    }
}
