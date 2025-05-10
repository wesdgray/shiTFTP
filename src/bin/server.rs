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

