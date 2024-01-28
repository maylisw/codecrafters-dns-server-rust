use std::vec;

use super::*;
#[test]
fn test_to_from_header() {
    let my_header = Header {
        id: 1265,
        resp: true,
        opcode: 15,
        authoratitive: true,
        truncated: true,
        recurse: true,
        recursion_avaliable: true,
        reserved: 7,
        rcode: 15,
        question_count: 256,
        answer_count: 357,
        ns_count: 328,
        additional_count: 421,
    };
    let mut buf = [0; 12];
    my_header.to_buf(&mut buf).unwrap();
    let res_header = Header::from_buf(&buf).unwrap();

    assert_eq!(my_header, res_header);
}

#[test]
fn test_to_from_packet() {
    let my_header = Header {
        id: 1265,
        resp: true,
        opcode: 15,
        authoratitive: true,
        truncated: true,
        recurse: true,
        recursion_avaliable: true,
        reserved: 7,
        rcode: 15,
        question_count: 1,
        answer_count: 1,
        ns_count: 328,
        additional_count: 421,
    };

    let my_packet = Packet {
        header: my_header,
        questions: vec![Question {
            names: vec![String::from("google"), String::from("com")],
            q_type: 1,
            class: 1,
        }],
        answers: vec![Answer {
            names: vec![String::from("google"), String::from("com")],
            a_type: 1,
            class: 1,
            ttl: 60,
            len: 4,
            data: [127, 0, 0, 1],
        }],
    };
    let packet = my_packet.clone();

    let mut buf = [0; 512];
    packet.to_buf(&mut buf).unwrap();
    let res_packet = Packet::from_buf(&buf).unwrap();
    // panic!("{:#?}", res_packet.questions);

    assert_eq!(my_packet, res_packet);
}
