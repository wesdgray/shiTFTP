use log::{debug, error, info, warn};

/// Message has the following binary format
///
/// # Read/Write
///  
///  2 bytes     string    1 byte     string   1 byte
///  ------------------------------------------------
/// | Opcode |  Filename  |   0  |    Mode    |   0  |
///  ------------------------------------------------
///  
///  # Data
///  Data receives 512 Bytes in the data field each time.
///  If < 512 Bytes are received, then this signals EOF.
///  
///  2 bytes     2 bytes      n bytes
///  ----------------------------------
/// | Opcode |   Block #  |   Data     |
///  ----------------------------------
///  
///  # Ack
///  
///  2 bytes     2 bytes
///  ---------------------
/// | Opcode |   Block #  |
///  ---------------------
///  
///  # Error
///  
///  2 bytes     2 bytes      string    1 byte
///  -----------------------------------------
/// | Opcode |  ErrorCode |   ErrMsg   |   0  |
///  -----------------------------------------
///  
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

fn decode_rw_message(bytes: &[u8]) -> Result<(String, Mode), ()> {
    let maybe_args: Vec<_> = bytes
        .split(|x| *x == 0x0)
        .filter(|x| !x.is_empty())
        .collect();
    if maybe_args.len() != 2 {
        error!("Expected 2 arguments but found: {}", maybe_args.len());
        error!("Value: {:?}", maybe_args);
        return Err(());
    }
    let filename = String::from_utf8(maybe_args[0].to_vec()).unwrap();
    let mode = maybe_args[1].try_into()?;
    Ok((filename, mode))
}

impl TryFrom<&[u8]> for Message {
    type Error = ();
    fn try_from(value: &[u8]) -> Result<Self, <Message as TryFrom<&[u8]>>::Error> {
        let (op_code, message) = value.split_at(2);
        debug!("op_code: {:?}", op_code);
        debug!("message: {:?}", message);
        match u16::from_be_bytes(op_code.try_into().map_err(|_| ())?) {
            Message::READ => {
                debug!("Read message: {:?}", message);
                let (filename, mode) = decode_rw_message(message)?;
                Ok(Message::Read { filename, mode })
            }
            Message::WRITE => {
                debug!("Write message: {:?}", message);
                let (filename, mode) = decode_rw_message(message)?;
                Ok(Message::Write { filename, mode })
            }
            op_code => {
                warn!("`{:?}` is not a valid op_code", op_code);
                Err(())
            }
        }
    }
}

// impl<'a> TftpMessage<'a> for ReadMessage {
//     const OPCODE: u16 = 0x00;
//
//     fn encode(&self) -> Vec<u8> {
//         debug!(
//             "Message.encode: Operation: Read, filename:{}, mode: {:?}",
//             self.filename, self.mode
//         );
//         let mut c: Vec<u8> = vec![];
//         c.extend(Self::OPCODE.to_be_bytes());
//         c.extend(self.filename.clone().into_bytes());
//         c.push(0x0);
//         c.extend(self.mode.to_string().into_bytes());
//         c.push(0x0);
//         c
//     }
// }

impl Message {
    pub const READ: u16 = 0;
    pub const WRITE: u16 = 1;
    pub const DATA: u16 = 2;
    pub const ACK: u16 = 3;
    pub const ERROR: u16 = 4;

    pub fn op_code(&self) -> u16 {
        match &self {
            Message::Read { .. } => Self::READ,
            Message::Write { .. } => Self::WRITE,
            Message::Data { .. } => Self::DATA,
            Message::Ack { .. } => Self::ACK,
            Message::Error { .. } => Self::ERROR,
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        match self {
            msg @ Message::Read { filename, mode } => {
                debug!(
                    "Message.encode: Operation: Read, filename:{}, mode: {:?}",
                    filename, mode
                );
                let mut c: Vec<u8> = vec![];
                c.extend(msg.op_code().to_be_bytes());
                c.extend(filename.clone().into_bytes());
                c.push(0x0);
                c.extend(mode.to_string().into_bytes());
                c.push(0x0);
                c
            }
            msg @ Message::Write { filename, mode } => {
                debug!(
                    "Message.encode: Operation: Write, filename:{}, mode: {:?}",
                    filename, mode
                );
                let mut c: Vec<u8> = vec![];
                c.extend(msg.op_code().to_be_bytes());
                c.extend(filename.clone().into_bytes());
                c.push(0x0);
                c.extend(mode.to_string().into_bytes());
                c.push(0x0);
                c
            }
            _ => {
                error!("not impl!");
                vec![]
            }
        }
    }
}

pub enum ErrorCode {
    Generic = 0x00,
    FileNotFound = 0x01,
    AccessViolation = 0x02,
    DiskFull = 0x03,
    IllegalOperation = 0x04,
    UnknownTransferId = 0x05,
    FileExists = 0x06,
    NoSuchUser = 0x07,
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    NetAscii,
    Octet,
    Mail,
}

impl Mode {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        match bytes {
            b"netascii" => Some(Mode::NetAscii),
            b"octet" => Some(Mode::Octet),
            b"mail" => Some(Mode::Mail),
            _ => None,
        }
    }
}

impl TryFrom<&[u8]> for Mode {
    type Error = ();
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value {
            b"netascii" => Ok(Mode::NetAscii),
            b"octet" => Ok(Mode::Octet),
            b"mail" => Ok(Mode::Mail),
            _ => Err(()),
        }
    }
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
    fn get_read_msg() -> Message {
        Message::Read {
            filename: "foo".to_string(),
            mode: Mode::NetAscii,
        }
    }

    fn get_write_msg() -> Message {
        Message::Write {
            filename: "foo".to_owned(),
            mode: Mode::NetAscii,
        }
    }

    // Read Message with filename: foo and mode: NetAscii
    const READ_MESSAGE_U8: &[u8] = &[
        0, 0, 102, 111, 111, 0, 110, 101, 116, 97, 115, 99, 105, 105, 0,
    ];

    // Write Message with filename: foo and mode: NetAscii
    const WRITE_MESSAGE_U8: &[u8] = &[
        0, 1, 102, 111, 111, 0, 110, 101, 116, 97, 115, 99, 105, 105, 0,
    ];

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
        debug!(
            "test: Read Message Encoding as: {:?}",
            get_read_msg().encode()
        );
        assert_eq!(READ_MESSAGE_U8, get_read_msg().encode());
    }

    #[test]
    fn test_encode_write() {
        let msg = get_write_msg();
        let bytes = WRITE_MESSAGE_U8;
        debug!("{:?}", msg);
        assert_eq!(Message::encode(&msg), bytes)
    }

    #[test]
    fn test_read_message_try_from_u8_slice() {
        debug!("{:?}", &READ_MESSAGE_U8);
        if let Ok(a) = TryInto::<Message>::try_into(READ_MESSAGE_U8) {
            debug!("Successfully decoded: {:?}", a);
            assert_eq!(a, get_read_msg());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_write_message_try_from_u8_slice() {
        debug!("{:?}", &WRITE_MESSAGE_U8);
        if let Ok(a) = TryInto::<Message>::try_into(WRITE_MESSAGE_U8) {
            debug!("Successfully decoded: {:?}", a);
            assert_eq!(a, get_write_msg());
        } else {
            assert!(false);
        }
    }
}
