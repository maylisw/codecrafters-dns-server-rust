//////////// MACROS ////////////

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
