#[macro_use]
mod macros;

#[cfg(test)]
mod tests;

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
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Question {
    names: Vec<String>,
    q_type: u16,
    class: u16,
}

impl Question {
    fn from_buf(buf: &[u8], num_questions: u16) -> Result<Vec<Question>, String> {
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

    fn to_buf(questions: Vec<Question>, buf: &mut [u8]) -> usize {
        let mut index = 0;
        for question in questions {
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
        return index;
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Answer {
    names: Vec<String>,
    a_type: u16,
    class: u16,
    ttl: u32,
    len: u16,
    data: [u8; 4],
}

impl Answer {
    pub fn to_buf(answers: Vec<Answer>, buf: &mut [u8]) -> usize {
        let mut index = 0;
        for answer in answers {
            for name in answer.names {
                buf[index] = name.len() as u8;
                index += 1;
                for b in name.into_bytes() {
                    buf[index] = b;
                    index += 1;
                }
            }
            buf[index] = 0;
            index += 1;

            (buf[index], buf[index + 1]) = le_u16_to_u8s!(answer.a_type);
            index += 2;
            (buf[index], buf[index + 1]) = le_u16_to_u8s!(answer.class);
            index += 2;

            buf[index..index + 4].clone_from_slice(&answer.ttl.to_be_bytes());
            index += 4;
            buf[index..index + 2].clone_from_slice(&answer.len.to_be_bytes());
            index += 2;
            buf[index..index + answer.len as usize].clone_from_slice(&answer.data);
            index += answer.len as usize;
        }
        return index;
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Packet {
    header: Header,
    questions: Vec<Question>,
    answers: Vec<Answer>,
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

        let questions = match Question::from_buf(&buf[12..], header.question_count) {
            Ok(value) => value,
            Err(error) => return Err(format!("error in Packet::from_buf: {}", error)),
        };

        return Ok(Packet {
            header: header,
            questions: questions,
            answers: vec![], //Todo: actually read the answers
        });
    }

    pub fn to_buf(self, buf: &mut [u8]) -> Result<(), String> {
        if buf.len() != 512 {
            return Err(format!(
                "Invalid Argument: length is {}, must be 512",
                buf.len()
            ));
        }
        // Header is fixed at 12 bytes
        let mut index = 12;

        match self.header.to_buf(&mut buf[0..index]) {
            Ok(()) => (),
            Err(err) => return Err(format!("error in Header::to_buf: {}", err)),
        };

        index += Question::to_buf(self.questions, &mut buf[index..]);
        index += Answer::to_buf(self.answers, &mut buf[index..]);

        return Ok(());
    }

    pub fn get_response(self) -> Result<Packet, String> {
        let rcode = match self.header.opcode {
            0 => 0,
            _ => 4,
        };
        let header = Header {
            id: self.header.id,
            resp: true,
            opcode: self.header.opcode,
            authoratitive: false,
            truncated: false,
            recurse: self.header.recurse,
            recursion_avaliable: false,
            reserved: 0,
            rcode: rcode,
            question_count: self.header.question_count,
            answer_count: 1,
            ns_count: 0,
            additional_count: 0,
        };

        let answer = Answer {
            names: self.questions[0].names.clone(),
            a_type: 1,
            class: 1,
            ttl: 60,
            len: 4,
            data: [127, 0, 0, 1],
        };
        return Ok(Packet {
            header: header,
            questions: self.questions,
            answers: vec![answer],
        });
    }
}
