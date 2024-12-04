pub struct PsPacketHeader {
    pub start_code: [u8; 4],
    pub scr: [u8; 6],
    pub program_mux_rate: [u8; 6],
    pub pack_stuffing_length: u8,
}