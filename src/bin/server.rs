use log::{debug, error, info, warn};
use shiTFTP::Message;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read};
use std::net::{SocketAddr, UdpSocket};
use std::thread;

fn main() {
    let _ = env_logger::builder()
        .format_file(true)
        .format_line_number(true)
        .try_init();

    info!("Starting Server");
    info!("Current directory: {:?}", std::env::current_dir());
    let mut buf = [0_u8; 1024];
    let socket = UdpSocket::bind("127.0.0.1:69").expect("Bind on the port");
    let mut threads = ThreadJoiner::new();

    loop {
        let (bytes_read, sender) = socket.recv_from(&mut buf).unwrap();
        debug!("bytes_read: {}", bytes_read);

        let read = buf.to_vec();
        debug!("Vec len {} | values: {:?}", read.len(), read);
        debug!("As UTF8: {}", String::from_utf8(read).unwrap());

        let copy = buf.clone();
        if let Ok(message) = copy.as_slice().try_into() {
            let handle = thread::spawn(move || {
                handle_transfer(message, sender);
            });
            threads.handles.push(handle);
        } else {
            warn!("Received invalid message!");
            debug!("Message content: {:?}", &buf);
        }
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

fn handle_transfer(message: Message, remote_addr: SocketAddr) {
    debug!("Remote Address: {:?}", remote_addr);
    match message {
        Message::Read {
            ref filename,
            mode: _,
        } => {
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
        Message::Write {
            ref filename,
            mode: _,
        } => {
            info!("Starting Write request for filename: {}", filename);
        }
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
