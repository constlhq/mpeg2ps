use bitfield::bitfield;

bitfield! {
	struct PesPacketHeaderFlags(u16);
    impl Debug;
    u8;
	get_orignal_or_copy,_: 0;
	get_copyright,_: 0;
	get_data_alignment_indicator,_: 0;
	get_pes_priority,_: 0;
	get_pes_scrambling_control,_: 5,4;
	get_reserved,_: 7,6;
	get_pes_extension_flag,_: 0;
	get_pes_crc_flag,_: 0;
	get_additional_copy_info_flag,_: 0;
	get_dsm_trick_mode_flag,_: 0;
	get_es_rate_flag,_: 0;
	get_escr_flag,_: 0;
	get_pts_dts_flags,_:7,6 ;

}
