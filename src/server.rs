use log::{debug, error, info, warn};
use std::fs::File;
use std::net::SocketAddr;
use std::{net::UdpSocket, path::PathBuf, thread};

use crate::protocol::Mode;
use crate::protocol::Message;

struct Server {
    socket: UdpSocket,
    receive_dir: PathBuf,
    send_dir: PathBuf,
    mode: Mode,
    threads: ThreadJoiner,
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


fn read_request(filename: String, mode: Mode) {
    info!("Starting Read request for filename: {}", filename);
    // TODO: Need to disallow .. and absolute paths
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    socket.connect(remote_addr).unwrap();

    let mut buf = [0_u8; 512];
    let mut block_num = 1;
    let mut ack = [0_u8; 4];

    'disk_read: loop {
        let bytes_read = reader.read(&mut buf).unwrap();
        let mut data = buf.to_vec();
        data.truncate(bytes_read);

        let data = Message::Data {
            block_num,
            data,
        };
        debug!("Read {bytes_read} bytes from disk");

        // TODO: some kind of timeout
        'net_send: loop {
            let _ = socket.send(data.encode().as_slice()).unwrap();
            debug!("Waiting for Ack on block_num: {block_num}");
            let _ = socket.recv(&mut ack);
            if let Ok(Message::Ack {
                block_num: ack_block_num,
            }) = TryInto::<Message>::try_into(&ack as &[u8])
            {
                if block_num == ack_block_num {
                    break 'net_send;
                }
            } else {
                debug!("Received invalid message: {:?}", ack);
            }
        }

        if bytes_read < 512 {
            info!("Read less than 512 bytes, transfer complete");
            break 'disk_read;
        }

        block_num += 1;
    }
}

fn write_request(filename: String, mode: Mode) {
    info!("Filename: {filename}, mode: {mode:?}");
}

fn handle_transfer(message: Message, remote_addr: SocketAddr) {
    debug!("Remote Address: {:?}", remote_addr);
    match message {
        Message::Read { filename, mode, } => read_request(filename, mode),
        Message::Write { filename, mode, } => write_request(filename, mode), 
        _ => {
            // TODO: Send an error back to the client
            error!("Transfers must be either be a Message::Read or Message::Write!");
        }
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
