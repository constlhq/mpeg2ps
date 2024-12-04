use bitfield::bitfield;

bitfield! {
   struct PsMap(MSB0[u8]);
	impl Debug;
    u32, get_start_code,_:0,31;				// == 0x000001BC
	u16, get_header_length,_:47,32;			// == 6 + es_map_length
	u8, get_ps_map_version,_: 52,48;			// == 0
	u8, get_reserved1,_: 54,53;				// == 3
	bool,get_current_next_indicator,_: 0;	// == 1
	bool,get_marker_bit,_: 0;				// == 1
	u8,	 get_reserved2,_: 63,57;				// == 127
	u16, get_ps_info_length,_:79,64;			// == 0
	u16, get_es_map_length,_:95,80;			// == 4 * es_num

}