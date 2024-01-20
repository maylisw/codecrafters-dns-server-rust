use super::*;
#[test]
fn test_to_from() {
    let my_packet = Packet {
        id: 1234,
        resp: true,
        opcode: 2,
        authoratitive: true,
        truncated: true,
        recurse: true,
        recursion_avaliable: true,
        reserved: 2,
        rcode: 34,
        question_count: 1,
        answer_count: 6,
        ns_count: 2,
        additional_count: 1,
    };
    let mut buf = [0; 512];
    my_packet.into_buf(&mut buf).unwrap();

    let res_packet = Packet::from_buf(&buf).unwrap();

    assert_eq!(my_packet, res_packet)
}
