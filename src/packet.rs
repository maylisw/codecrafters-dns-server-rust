use std::fmt;

#[macro_use]
mod macros;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Eq)]
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
    pub fn from_buf(buf: &[u8]) -> Result<Packet, String> {
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

    pub fn into_buf(&self, buf: &mut [u8]) -> Result<(), String> {
        if buf.len() != 512 {
            return Err(format!(
                "Invalid Argument: length is {}, must be 512",
                buf.len()
            ));
        }

        return Ok(());
    }

    pub fn get_response(&self) -> Result<Packet, String> {
        return Ok(Packet {
            id: self.id,
            resp: true,
            opcode: 0,
            authoratitive: false,
            truncated: false,
            recurse: false,
            recursion_avaliable: false,
            reserved: 0,
            rcode: 0,
            question_count: 0,
            answer_count: 0,
            ns_count: 0,
            additional_count: 0,
        });
    }
}
