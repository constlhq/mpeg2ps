use bitfield::bitfield;

bitfield! {
   struct PsSystemHeaderStreamTable(MSB0[u8]);
	impl Debug;
	u8,get_stream_id,_:7,0;
	u8,get_p_std_buffer_size_bound_high5,_: 15,8;
		 // 0 for audio, scale x128B
		 // 1 for video, scale x1024B
	bool,	 get_p_std_buffer_bound_scale,_: 0;
	u8,	 get_reserved,_: 2;
	u8,get_p_std_buffer_size_bound_low8,_:23,16;
}
