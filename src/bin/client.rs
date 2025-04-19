use std::net::UdpSocket;
use tftp_rs::*;

fn main() {
<<<<<<< Updated upstream
    let buf = "123456789".as_bytes();
    let socket = UdpSocket::bind("127.0.0.1:42069").unwrap();
    socket.connect("127.0.0.1:69").unwrap();
    let sent = socket.send(buf).unwrap();
    let read_msg = Message::Read {
        filename: "my_file".to_owned(),
        mode: Mode::NetAscii,
    };
    let enc_read_msg = read_msg.encode();
    println!("{:?}", enc_read_msg.clone());
    let sent = socket.send(enc_read_msg.as_slice()).unwrap();
    println!("{}", sent);
=======
    let cli = Cli::parse();
    let _ = env_logger::builder()
        .format_file(true)
        .format_line_number(true)
        .try_init();

    match &cli.commands {
        Commands::Read { filename, server } => {
            info!("Read the file: {} from {}", filename, server);

            // Network Request to initiate the read
            let socket = UdpSocket::bind("127.0.0.0:0").unwrap();
            let read_req = Message::Read {
                filename: filename.to_string(),
                mode: Mode::NetAscii,
            };
            let recv = socket
                .send_to(read_req.encode().as_slice(), server)
                .unwrap();
            
            // Setup for writing
            let mut block_num = 1;
            let mut buf = [0_u8; 2 + 2 + 512];
            let file = File::create(filename).unwrap();
            let mut writer = BufWriter::new(file);

            loop {
                let (read_bytes, recv_from) = socket.recv_from(&mut buf).unwrap();
                debug!("Transfer socket is: {recv_from}");
                debug!("Bytes read: {}", read_bytes);

                if let Ok(Message::Data {
                    block_num: ack_block_num,
                    data,
                }) = buf.as_slice().try_into()
                {
                    debug!("Received data for block_num: {block_num}");
                    if block_num == ack_block_num {
                        let _ = writer.write(&data).unwrap();
                        let ack_msg = Message::Ack { block_num };
                        // TODO: Ensure the ack was received before incrementing block_num
                        let _ = socket
                            .send_to(ack_msg.encode().as_slice(), recv_from)
                            .unwrap();
                        block_num += 1;
                    }
                    
                    if read_bytes < 512 {
                        info!("Read less than 512 bytes, transfer complete");
                        break;
                    }
                } else {
                    debug!("Received bad message: {:?}", buf);
                }
            }
        }

        Commands::Write { filename, server } => {
            info!("Write the file: {} to {}", filename, server);
        }
    }
>>>>>>> Stashed changes
}
