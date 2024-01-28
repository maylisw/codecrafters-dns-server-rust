use std::collections::HashMap;
use std::net::UdpSocket;

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
    fn from_buf(
        buf: &[u8],
        start_index: usize,
        num_questions: u16,
    ) -> Result<(Vec<Question>, usize), String> {
        let mut index = start_index;
        // Maps an index in the buffer to the index of the question and name associated w/
        // that index in the buffer as such:
        // {index in buf : (index of question, index of name)}
        let mut index_to_question_name = HashMap::<usize, (usize, usize)>::new();
        let mut questions = Vec::<Question>::new();

        for _ in 0..num_questions {
            let mut names = Vec::<String>::new();
            loop {
                let len: usize = buf[index] as usize;
                index += 1;
                if len == 0 {
                    break;
                }
                if compressed!(len) {
                    let buf_idx = get_compressed_index!(len, buf[index]) as usize;
                    index += 1;

                    let (q_idx, n_idx) = match index_to_question_name.get(&buf_idx) {
                        Some((q_idx, n_idx)) => (q_idx, n_idx),
                        None => {
                            return Err(format!(
                                "compressed index: {}, not in index_to_question_name",
                                buf_idx
                            ))
                        }
                    };
                    let names_sub = questions[*q_idx].names[*n_idx..].to_vec().clone();
                    names.extend(names_sub);
                } else {
                    index_to_question_name.insert(index - 1, (questions.len(), names.len()));
                    let value = match String::from_utf8(buf[index..index + len].to_vec()) {
                        Ok(value) => value,
                        Err(err) => return Err(format!("error in String::from_utf8: {}", err)),
                    };
                    names.push(value);
                    index += len;
                }
            }

            questions.push(Question {
                names: names,
                q_type: be_u8s_to_u16!(&buf[index..index + 2]),
                class: be_u8s_to_u16!(&buf[index + 2..index + 4]),
            });

            index += 4;
        }

        return Ok((questions, index));
    }

    fn to_buf(questions: Vec<Question>, start_index: usize, buf: &mut [u8]) -> usize {
        let mut index = start_index;
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
    fn from_buf(buf: &[u8], start_index: usize, num_answers: u16) -> Result<Vec<Answer>, String> {
        let mut index = start_index;
        let mut answers = Vec::<Answer>::new();

        for _ in 0..num_answers {
            let mut names = Vec::<String>::new();
            loop {
                let len: usize = buf[index] as usize;
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
            let a_type = be_u8s_to_u16!(&buf[index..index + 2]);
            index += 2;
            let class = be_u8s_to_u16!(&buf[index..index + 2]);
            index += 2;
            let ttl = be_u8s_to_u32!(&buf[index..index + 4]);
            index += 4;
            let a_len = match be_u8s_to_u16!(&buf[index..index + 2]) {
                4 => 4,
                l => {
                    return Err(format!(
                        "error ip address must be length 4, found length {}",
                        l
                    ))
                }
            };
            index += 2;
            let mut data = [0; 4];
            data.clone_from_slice(&buf[index..index + 4]);
            index += 4;

            answers.push(Answer {
                names: names,
                a_type: a_type,
                class: class,
                ttl: ttl,
                len: a_len,
                data: data,
            });
            index += 4;
        }

        return Ok(answers);
    }

    fn to_buf(answers: Vec<Answer>, buf: &mut [u8]) -> usize {
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

        let (questions, index) = match Question::from_buf(&buf, 12, header.question_count) {
            Ok(value) => value,
            Err(error) => return Err(format!("error in Packet::from_buf: {}", error)),
        };

        let answers = match Answer::from_buf(&buf, index, header.answer_count) {
            Ok(value) => value,
            Err(error) => return Err(format!("error in Packet::from_buf: {}", error)),
        };

        return Ok(Packet {
            header: header,
            questions: questions,
            answers: answers, //Todo: actually read the answers
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

        index = Question::to_buf(self.questions, index, &mut buf[0..]);
        index += Answer::to_buf(self.answers, &mut buf[index..]);

        return Ok(());
    }

    pub fn get_response(
        self,
        socket: &UdpSocket,
        resolver_address: &String,
    ) -> Result<Packet, String> {
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
            answer_count: self.header.question_count,
            ns_count: 0,
            additional_count: 0,
        };

        let mut answers = Vec::<Answer>::new();

        for q in self.questions.clone() {
            let send_packet = Packet {
                header: self.header.clone(),
                questions: vec![q],
                answers: vec![],
            };

            // SEND PACKET

            let mut my_q = [0; 512];
            send_packet
                .to_buf(&mut my_q)
                .expect("error with send_packet::to_buf()");

            socket
                .send_to(&my_q, resolver_address)
                .expect(&format!("failed to send response to {}", resolver_address));

            // RECIEVE PACKET

            let result = match socket.recv_from(&mut my_q) {
                Ok((size, source)) => {
                    println!("Received {} bytes from {}", size, source);

                    match Packet::from_buf(&my_q) {
                        Ok(answer) => answer,
                        Err(err) => return Err(format!("error in Packet::from_buf: {}", err)),
                    }
                }
                Err(e) => return Err(format!("error receiving data: {}", e)),
            };

            // STEAL ANSWERS

            for a in result.answers {
                answers.push(a.to_owned())
            }
        }
        return Ok(Packet {
            header: header,
            questions: self.questions,
            answers: answers,
        });
    }
}
