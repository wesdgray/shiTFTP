use log::{debug, error, info, warn};
use shiTFTP::Message;
use std::net::{SocketAddr, UdpSocket};
use std::thread;

fn main() {
    let _ = env_logger::builder()
        .format_file(true)
        .format_line_number(true)
        .try_init();
    debug!("Starting Server");
    let mut buf = [0_u8; 1024];
    let socket = UdpSocket::bind("127.0.0.1:69").expect("Bind on the port");
    let mut threads = ThreadJoiner::new();
    loop {
        let (bytes_read, sender) = socket.recv_from(&mut buf).unwrap();
        debug!("bytes_read: {}", bytes_read);

        let read = buf.to_vec();
        debug!("Vec len {} | values: {:?}", read.len(), read);
        debug!("{}", String::from_utf8(read.clone()).unwrap());

        // let mut op_code: [u8; 2];
        // op_code.clone_from_slice(&buf[0..2]);
        // if let op_code = u16::from_be_bytes(op_code) {
        //     match op_code {
        //         Message::WRITE => {
        //             let req_socket = UdpSocket::bind("127.0.0.1:0");
        //             let remote_addr = req_socket.unwrap().local_addr().unwrap();
        //             threads.handles.push(thread::spawn(handle_write(remote_addr)));
        //         }
        //         _ => {
        //             warn!("{:?} tried to initiate with a non R/W request", sender);
        //             warn!("Decoded as {:?}", msg);
        //         }
        //     }
        // }
    }
}

struct ThreadJoiner {
    handles: Vec<thread::JoinHandle<()>>,
}

impl ThreadJoiner {
    fn new() -> Self {
        ThreadJoiner { handles: vec![] }
    }
}

impl Drop for ThreadJoiner {
    fn drop(&mut self) {
        while let Some(handle) = self.handles.pop() {
            let _ = handle.join();
        }
    }
}

fn handle_write(remote_addr: SocketAddr) -> impl Fn() {
    move || {
        debug!("Remote Address: {:?}", remote_addr);
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
