//////////// FROM MACROS ////////////

macro_rules! be_u8s_to_u16 {
    ($x:expr) => {
        (($x[0] as u16) << 8 | ($x[1] as u16))
    };
}

macro_rules! extract_resp {
    ($x:expr) => {
        ($x & 1 << 7) != 0
    };
}

macro_rules! extract_opcode {
    ($x:expr) => {
        ($x & 0b1111 << 3) >> 3
    };
}

macro_rules! extract_authoratitive {
    ($x:expr) => {
        ($x & 1 << 2) != 0
    };
}

macro_rules! extract_truncated {
    ($x:expr) => {
        ($x & 1 << 1) != 0
    };
}

macro_rules! extract_recurse {
    ($x:expr) => {
        ($x & 1) != 0
    };
}

macro_rules! extract_recursion_avaliable {
    ($x:expr) => {
        ($x & 1 << 7) != 0
    };
}

macro_rules! extract_reserved {
    ($x:expr) => {
        ($x & 0b111 << 4) >> 4
    };
}

macro_rules! extract_rcode {
    ($x:expr) => {
        ($x & 0b1111)
    };
}

//////////// TO MACROS ////////////
macro_rules! le_u16_to_u8s {
    ($x:expr) => {
        (($x >> 8) as u8, ($x as u8))
    };
}

macro_rules! pack_qr_opcode_aa_tc_rd {
    ($u:expr,$w:expr,$x:expr,$y:expr,$z:expr) => {
        ((($u as u8) << 7) | ($w << 3) | ($x as u8) << 2 | ($y as u8) << 1 | ($z as u8))
    };
}

macro_rules! pack_ra_reserved_rcode {
    ($x:expr,$y:expr,$z:expr) => {
        ((($x as u8) << 7) | ($y << 4) | $z)
    };
}
