use crate::es::StreamId;
use crate::Result;
use bitfield::bitfield;
use std::io::Read;

bitfield! {
    pub struct PsSystemHeaderStreamTable(MSB0 [u8]);
        impl Debug;
        pub u8, stream_id,set_stream_id: 7,0;
        pub u8, one_one,_: 9,8;
        pub bool,p_std_buffer_bound_scale,set_p_std_buffer_bound_scale: 10;
        pub u16,p_std_buffer_size_bound,set_p_std_buffer_size_bound: 23,11;
}

impl PsSystemHeaderStreamTable<[u8; 3]> {
    pub(super) fn read_from<R: Read>(mut reader: R) -> Result<Self> {
        let mut buf = [0; 3];
        reader.read_exact(&mut buf);
        let system_header_stream_table = PsSystemHeaderStreamTable(buf);
        Ok(system_header_stream_table)
    }
}
