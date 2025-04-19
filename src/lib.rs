use log::{debug, error, info, warn};
#[derive(Debug)]
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

#[derive(Debug)]
pub enum Mode {
    NetAscii,
    Octet,
    Mail,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn setup() {
        let _ = env_logger::try_init();
    }
    #[test]
    fn test_encode_read() {
        setup();
        let read = Message::Read {
            filename: "foo".to_string(),
            mode: Mode::NetAscii,
        };
        info!("test: Read Message Encoding as: {:?}", read.encode());
    }

    #[test]
    fn test_decode_read() {
        setup();
        let read = [0, 0, 102, 111, 111];
        let read_msg = Message::decode(&read);
        match read_msg {
            Ok(msg) => {
                info!("test: Read Message decoded: {:?}", msg);
            }
            Err(e) => {
                warn!("There was err: {:?}", e);
            }
        }
    }
}
