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
        block_num: usize,
        data: [u8; 512],
    },
    Ack {
        block_num: usize,
    },
    Error {
        error_code: usize,
        error_msg: String,
    },
}

impl Message {

    pub fn encode(&self) -> Vec<u8> {
        match self {
            Message::Read{filename, mode} => {
                println!("Op:{}, filename:{}, mode: {:?}", 0, filename, mode);
                let mut c: Vec<u8> = Vec::new();
                c.extend([0,0]);
                c.extend(filename.clone().into_bytes());
                c
            },
            _ => {
                println!("not impl!");
                vec![]
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

