pub mod packet;
use std::env;

use std::net::UdpSocket;

fn main() {
    println!("Logs from your program will appear here!");

    if env::args().len() != 3 {
        panic!("must provide a forwarding resolver")
    }

    let resolver_address = env::args().last().unwrap();

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");

    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                let query = match packet::Packet::from_buf(&buf) {
                    Ok(query) => query,
                    Err(err) => {
                        eprintln!("error in Packet::from_buf: {}", err);
                        continue;
                    }
                };
                println!("Recieved packet: {:#?}", query);

                let response = match query.get_response(&udp_socket, &resolver_address) {
                    Ok(response) => response,
                    Err(err) => {
                        eprintln!("error in Packet::get_response: {}", err);
                        continue;
                    }
                };
                println!("Responding with packet: {:#?}", response);

                match response.to_buf(&mut buf) {
                    Ok(()) => (),
                    Err(err) => {
                        eprintln!("error in Packet::into_buf: {}", err);
                        continue;
                    }
                };

                udp_socket
                    .send_to(&buf, source)
                    .expect("failed to send response");
            }
            Err(e) => {
                eprintln!("error receiving data: {}", e);
                break;
            }
        }
    }
}
