use std::fmt;

/*
HEADER
    id: 16 bits
    qr: 1 bit
    opcode: 4 bits
    aa: 1 bit
    tc: 1 bit
    rd: 1 bit
    ra: 1 bit
    reserved: 3 bits
    rcode: 4 bits
    qdcount: 16 bits
    ancount: 16 bits
    nscount: 16 bits
    arcount: 16 bits
*/

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

#[derive(Debug)]
pub struct Packet {
    id: u16,
    resp: bool,
    opcode: u8,
    authoratitive: bool,
    truncated: bool,
    recurse: bool,
    recursion_avaliable: bool,
    reserved: u8,
    rcode: u8,
    question_count: u16,
    answer_count: u16,
    ns_count: u16,
    additional_count: u16,
}

impl fmt::Display for Packet {
    // TODO: finish
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id: {}, resp: {}", self.id, self.resp)
    }
}

impl Packet {
    pub fn packet_id(&self) -> u16 {
        return self.id;
    }

    pub fn from(buf: &[u8]) -> Result<Packet, String> {
        if buf.len() != 512 {
            return Err(format!(
                "Invalid Argument: length is {}, must be 512",
                buf.len()
            ));
        }

        return Ok(Packet {
            id: be_u8s_to_u16!(&buf[0..2]),
            resp: extract_resp!(buf[2]),
            opcode: extract_opcode!(buf[2]),
            authoratitive: extract_authoratitive!(buf[2]),
            truncated: extract_truncated!(buf[2]),
            recurse: extract_recurse!(buf[2]),
            recursion_avaliable: extract_recursion_avaliable!(buf[3]),
            reserved: extract_reserved!(buf[3]),
            rcode: extract_rcode!(buf[3]),
            question_count: be_u8s_to_u16!(&buf[4..6]),
            answer_count: be_u8s_to_u16!(&buf[6..8]),
            ns_count: be_u8s_to_u16!(&buf[8..10]),
            additional_count: be_u8s_to_u16!(&buf[10..12]),
        });
    }
}
