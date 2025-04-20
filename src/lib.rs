use log::{debug, error, info, warn};
#[derive(Debug, PartialEq)]
pub enum Message {
    Read {
        filename: String,
        mode: Mode,
    },
    Write {
        filename: String,
        mode: Mode,
    },
    Data {
<<<<<<< Updated upstream
        block_num: usize,
        data: [u8; 512],
=======
        block_num: u16,
        data: Vec<u8>,
>>>>>>> Stashed changes
    },
    Ack {
        block_num: usize,
    },
    Error {
        error_code: usize,
        error_msg: String,
    },
}

pub enum State {
    Init,
    Read,
    Write,
    Done,
}

impl Message {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            Message::Read { filename, mode } => {
                info!(
                    "Message.encode: Operation:{}, filename:{}, mode: {:?}",
                    0, filename, mode
                );
                let mut c: Vec<u8> = Vec::new();
                c.extend([0, 0]);
                c.extend(filename.clone().into_bytes());
                c.extend(mode.to_string().into_bytes());
                c
            }
            _ => {
                error!("not impl!");
                vec![]
            }
        }
    }

    pub fn decode(buff: &[u8]) -> Result<Message, ()> {
        debug!("{:?}", buff);
        if buff.len() < 2 {
            warn!("len too short");
            return Err(());
        }
        let op_code = &buff[0..=1];
        match op_code {
            [0, 0] => Ok(Message::Read {
                filename: "foo".to_string(),
                mode: Mode::NetAscii,
            }),
            x => {
                debug!("{:?}", x);
                Err(())
            }
        }
    }
}
pub enum ErrorCode {
    Generic = 0x0,
    FileNotFound = 0x1,
    AccessViolation = 0x2,
    DiskFull = 0x3,
    IllegalOperation = 0x4,
    UnknownTransferId = 0x5,
    FileExists = 0x6,
    NoSuchUser = 0x7,
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    NetAscii,
    Octet,
    Mail,
}

impl ToString for Mode {
    fn to_string(&self) -> String {
        match self {
            Mode::NetAscii => "netascii".to_string(),
            Mode::Octet => "octet".to_string(),
            Mode::Mail => "mail".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[ctor::ctor]
    fn init_logger() {
        let _ = env_logger::builder()
            .format_file(true)
            .format_line_number(true)
            .is_test(true)
            .try_init();
    }
    #[test]
    fn test_encode_read() {
        let read = Message::Read {
            filename: "foo".to_string(),
            mode: Mode::NetAscii,
        };
        debug!("test: Read Message Encoding as: {:?}", read.encode());
        assert_eq!(vec![0, 0, 102, 111, 111, 110, 101, 116, 97, 115, 99, 105, 105], read.encode());
    }

    #[test]
    fn test_decode_read() {
        // Read Message with filename: foo and mode: NetAscii
        let read = [0, 0, 102, 111, 111];
        let read_msg = Message::decode(&read);
        match read_msg {
            Ok(msg) => {
                debug!("test: Read Message decoded: {:?}", msg);
            }
            Err(e) => {
                debug!("There was err: {:?}", e);
            }
        }
        assert_eq!(
            Message::Read {
                filename: "foo".to_owned(),
                mode: Mode::NetAscii
            },
            Message::decode(&read).unwrap()
        );
    }
    #[test]
    fn test_encode_write() {
        let write = Message::Write{filename: "foo".to_owned(), mode: Mode::NetAscii};
        debug!("{:?}", write);
    }

    #[test]
    fn test_decode_write() {}

    #[test]
    fn test_encode_data() {}

    #[test]
    fn test_decode_data() {}

    #[test]
    fn test_encode_ack() {}

    #[test]
    fn test_decode_ack() {}

    #[test]
    fn test_encode_error() {}

    #[test]
    fn test_decode_error() {}
}
