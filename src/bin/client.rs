use std::net::UdpSocket;
use tftp_rs::*;

fn main() {
    let buf = "123456789".as_bytes();
    let socket = UdpSocket::bind("127.0.0.1:42069").unwrap();
    socket.connect("127.0.0.1:69").unwrap();
    let sent = socket.send(buf).unwrap();
    let read_msg = Message::Read{filename: "my_file".to_owned(), mode: Mode::NetAscii};
    let enc_read_msg = read_msg.encode();
    println!("{:?}", enc_read_msg.clone());
    let sent = socket.send(enc_read_msg.as_slice()).unwrap();
    println!("{}", sent);
}
