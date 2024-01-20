pub mod packet;

use std::net::UdpSocket;

fn main() {
    println!("Logs from your program will appear here!");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");

    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                let query = match packet::Packet::from_buf(&buf) {
                    Ok(query) => query,
                    Err(err) => panic!("error in Packet::from_buf {}", err),
                };
                println!("Recieved packet: {:#?}", query);

                let response = match query.get_response() {
                    Ok(response) => response,
                    Err(err) => panic!("error in Packet::get_response {}", err),
                };
                println!("Responding with packet: {:#?}", response);

                match response.into_buf(&mut buf) {
                    Ok(()) => (),
                    Err(err) => panic!("error in Packet::into_buf {}", err),
                };

                udp_socket
                    .send_to(&buf, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
