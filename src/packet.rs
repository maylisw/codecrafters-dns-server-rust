#[macro_use]
mod macros;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Question {
    names: Vec<String>,
    q_type: u16,
    class: u16,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Header {
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

impl Header {
    fn from_buf(buf: &[u8]) -> Result<Header, String> {
        if buf.len() != 12 {
            return Err(format!(
                "Invalid Argument: length is {}, must be 12",
                buf.len()
            ));
        }

        return Ok(Header {
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

    fn to_buf(&self, buf: &mut [u8]) -> Result<(), String> {
        if buf.len() != 12 {
            return Err(format!(
                "Invalid Argument: length is {}, must be 12",
                buf.len()
            ));
        }

        (buf[0], buf[1]) = le_u16_to_u8s!(self.id);
        buf[2] = pack_qr_opcode_aa_tc_rd!(
            self.resp,
            self.opcode,
            self.authoratitive,
            self.truncated,
            self.recurse
        );
        buf[3] = pack_ra_reserved_rcode!(self.recursion_avaliable, self.reserved, self.rcode);
        (buf[4], buf[5]) = le_u16_to_u8s!(self.question_count);
        (buf[6], buf[7]) = le_u16_to_u8s!(self.answer_count);
        (buf[8], buf[9]) = le_u16_to_u8s!(self.ns_count);
        (buf[10], buf[11]) = le_u16_to_u8s!(self.additional_count);
        return Ok(());
    }

    pub fn get_response(self) -> Header {
        return Header {
            id: self.id,
            resp: true,
            opcode: 0,
            authoratitive: false,
            truncated: false,
            recurse: false,
            recursion_avaliable: false,
            reserved: 0,
            rcode: 0,
            question_count: self.question_count,
            answer_count: 0,
            ns_count: 0,
            additional_count: 0,
        };
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Packet {
    header: Header,
    questions: Vec<Question>,
}

impl Packet {
    pub fn from_buf(buf: &[u8]) -> Result<Packet, String> {
        if buf.len() != 512 {
            return Err(format!(
                "Invalid Argument: length is {}, must be 512",
                buf.len()
            ));
        }

        let header = match Header::from_buf(&buf[0..12]) {
            Ok(value) => value,
            Err(error) => return Err(format!("error in Header::from_buf: {}", error)),
        };

        let questions = match Packet::parse_questions(&buf[12..], header.question_count) {
            Ok(value) => value,
            Err(error) => return Err(format!("error in Packet::parse_questions: {}", error)),
        };

        return Ok(Packet {
            header: header,
            questions: questions,
        });
    }

    fn parse_questions(buf: &[u8], num_questions: u16) -> Result<Vec<Question>, String> {
        let mut index = 0;

        let mut questions = Vec::<Question>::new();

        for _i in 0..num_questions {
            let mut names = Vec::<String>::new();
            loop {
                let len = buf[index] as usize;
                index += 1;
                if len == 0 {
                    break;
                }

                let value = match String::from_utf8(buf[index..index + len].to_vec()) {
                    Ok(value) => value,
                    Err(err) => return Err(format!("error in String::from_utf8: {}", err)),
                };
                names.push(value);
                index += len;
            }

            questions.push(Question {
                names: names,
                q_type: dbg!(be_u8s_to_u16!(&buf[index..index + 2])),
                class: be_u8s_to_u16!(&buf[index + 2..index + 4]),
            });

            index += 4;
        }

        return Ok(questions);
    }

    pub fn to_buf(self, buf: &mut [u8]) -> Result<(), String> {
        if buf.len() != 512 {
            return Err(format!(
                "Invalid Argument: length is {}, must be 512",
                buf.len()
            ));
        }

        match self.header.to_buf(&mut buf[0..12]) {
            Ok(()) => (),
            Err(err) => return Err(format!("error in Header::to_buf: {}", err)),
        };

        let mut index = 12;
        for question in self.questions {
            for name in question.names {
                buf[index] = name.len() as u8;
                index += 1;
                for b in name.into_bytes() {
                    buf[index] = b;
                    index += 1;
                }
            }
            buf[index] = 0;
            index += 1;

            (buf[index], buf[index + 1]) = le_u16_to_u8s!(question.q_type);
            index += 2;
            (buf[index], buf[index + 1]) = le_u16_to_u8s!(question.class);
            index += 2;
        }

        return Ok(());
    }

    pub fn get_response(self) -> Result<Packet, String> {
        return Ok(Packet {
            header: self.header.get_response(),
            questions: self.questions,
        });
    }
}
