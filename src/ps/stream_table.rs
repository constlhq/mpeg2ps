use crate::es::StreamId;

pub struct PsSystemHeaderStreamTable {
    pub stream_id: u8,
    pub p_std_buffer_size_bound_high5: u8,
    // 0 for audio, scale x128B
    // 1 for video, scale x1024B
    pub p_std_buffer_bound_scale: bool,
    pub reserved: u8, // == 2b
    pub p_std_buffer_size_bound_low8: u8,
}
