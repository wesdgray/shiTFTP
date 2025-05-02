use crate::Message;
use clap::Parser;
use log::info;
use shiTFTP::*;
use std::{fs::File, io::{BufReader, BufWriter, Write}, net::{SocketAddr, ToSocketAddrs, UdpSocket}};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}
#[derive(Parser, Debug)]
enum Commands {
    Read {
        server: SocketAddr,
        filename: String,
    },
    Write {
        server: SocketAddr,
        filename: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.commands {
        Commands::Read { filename, server } => {
            info!("Read the file: {} from {}", filename, server);
            let read_req = Message::Read { filename: filename.to_string(), mode: Mode::NetAscii };
            let socket = UdpSocket::bind("127.0.0.0:0").unwrap();
            
            let recv = socket.send_to(read_req.encode().as_slice(), server).unwrap();
            let mut buf = [0_u8; 2+2+512];
            let mut block_num = 1;
            let file = File::create(filename).unwrap();
            let mut writer = BufWriter::new(file); 

            loop {
                let (read_bytes, recv_from) = socket.recv_from(&mut buf).unwrap();
                if let Ok(Message::Data{ block_num: ack_block_num, data }) = buf.as_slice().try_into() {
                    if block_num == ack_block_num {
                        let _ = writer.write(&data).unwrap();
                        let ack_msg = Message::Ack { block_num };
                        // TODO: Ensure the ack was received before incrementing block_num
                        let _ = socket.send_to(ack_msg.encode().as_slice(), recv_from).unwrap();
                        block_num += 1;
                    }

                    if read_bytes < 512 {
                        break;
                    }
                }
            }
        }

        Commands::Write { filename, server } => {
            println!("Write the file: {} to {}", filename, server);
        }
    }
    let buf = "123456789".as_bytes();
    let socket = UdpSocket::bind("127.0.0.1:42069").unwrap();
    socket.connect("127.0.0.1:69").unwrap();
    let sent = socket.send(buf).unwrap();
    let read_msg = Message::Write {
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

fn connect_to_server(addr: String) -> SocketAddr {
    addr.to_socket_addrs().unwrap();
    "127.0.0.1:69".parse().unwrap()
}

fn handle_read() {}

fn handle_write() {}
