#[cfg(test)]
mod tests {
    extern crate mdg;
    use self::mdg::constants::{
        BLOCK_LENGTH, DIGEST_LENGTH, INITIAL_STATE, PADDING, SHIFTS, T_VALUES,
    };

    #[test]
    fn test_block_length() {
        assert_eq!(BLOCK_LENGTH, 64);
    }

    #[test]
    fn test_digest_length() {
        assert_eq!(DIGEST_LENGTH, 16);
    }

    #[test]
    fn test_initial_state() {
        assert_eq!(
            INITIAL_STATE,
            [0x6745_2301, 0xefcd_ab89, 0x98ba_dcfe, 0x1032_5476]
        );
    }

    #[test]
    fn test_shifts() {
        assert_eq!(
            SHIFTS,
            [
                7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14,
                20, 5, 9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11,
                16, 23, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
            ]
        );
    }

    #[test]
    fn test_padding() {
        assert_eq!(
            PADDING,
            [
                0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]
        );
    }

    #[test]
    fn test_t_values() {
        assert_eq!(
            T_VALUES,
            [
                0xd76a_a478,
                0xe8c7_b756,
                0x2420_70db,
                0xc1bd_ceee,
                0xf57c_0faf,
                0x4787_c62a,
                0xa830_4613,
                0xfd46_9501,
                0x6980_98d8,
                0x8b44_f7af,
                0xffff_5bb1,
                0x895c_d7be,
                0x6b90_1122,
                0xfd98_7193,
                0xa679_438e,
                0x49b4_0821,
                0xf61e_2562,
                0xc040_b340,
                0x265e_5a51,
                0xe9b6_c7aa,
                0xd62f_105d,
                0x0244_1453,
                0xd8a1_e681,
                0xe7d3_fbc8,
                0x21e1_cde6,
                0xc337_07d6,
                0xf4d5_0d87,
                0x455a_14ed,
                0xa9e3_e905,
                0xfcef_a3f8,
                0x676f_02d9,
                0x8d2a_4c8a,
                0xfffa_3942,
                0x8771_f681,
                0x6d9d_6122,
                0xfde5_380c,
                0xa4be_ea44,
                0x4bde_cfa9,
                0xf6bb_4b60,
                0xbebf_bc70,
                0x289b_7ec6,
                0xeaa1_27fa,
                0xd4ef_3085,
                0x0488_1d05,
                0xd9d4_d039,
                0xe6db_99e5,
                0x1fa2_7cf8,
                0xc4ac_5665,
                0xf429_2244,
                0x432a_ff97,
                0xab94_23a7,
                0xfc93_a039,
                0x655b_59c3,
                0x8f0c_cc92,
                0xffef_f47d,
                0x8584_5dd1,
                0x6fa8_7e4f,
                0xfe2c_e6e0,
                0xa301_4314,
                0x4e08_11a1,
                0xf753_7e82,
                0xbd3a_f235,
                0x2ad7_d2bb,
                0xeb86_d391,
            ]
        );
    }
}
