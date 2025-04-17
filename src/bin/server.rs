use std::net::UdpSocket;

fn main() {
    println!("Starting Server");
    let mut buf = [0_u8; 1024];
    let socket = UdpSocket::bind("127.0.0.1:69").expect("Bind on the port");
    loop {
        let bytes_read = socket.recv(&mut buf).unwrap();
        println!("bytes_read: {}", bytes_read);
        let read = buf.to_vec();
        let msg = String::from_utf8(read.clone()).unwrap();
        println!("Vec len: {}", read.len());
        println!("Vec values: {:?}", read);
        println!("{}", msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_true() {
        assert!(true);
    }
}
fn handle_transfer() {}
