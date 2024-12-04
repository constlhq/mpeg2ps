use bitfield::bitfield;

bitfield! {
	struct PsSystemHeader(MSB0[u8]);
	impl Debug;
	u8;
	u32,get_start_code,_:31,0;
	u16, get_header_length,_: 47,32;
	u32, get_rate_bound,_: 71,48;
	bool,get_csps_flag,_:0;
	bool,get_fixed_flag,_:0;
	u8, get_audio_bound,_:79,74;
	u8, get_video_bound,_:84,80;
	bool,get_marker_bit,_: 0;
	bool, get_system_video_lock_flag,_: 0;
	bool,get_system_audio_lock_flag,_: 0;
	u8,get_reserved_byte,_:95,88;

}