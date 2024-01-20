use super::*;
#[test]
fn test_to_from() {
    let my_packet = Packet {
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
    let mut buf = [0; 512];
    my_packet.into_buf(&mut buf).unwrap();

    let res_packet = Packet::from_buf(&buf).unwrap();

    assert_eq!(my_packet, res_packet)
}
