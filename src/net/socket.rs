use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;

const MSG_SIZE_BYTES: usize = 8;
const CHECKSUM_BYTES: usize = 8;

pub struct NetSocket {
    pub id: u64,
    stream: TcpStream,
    buffer: [u8; 512],
    len: usize,
    msg_size: Option<usize>,
}

impl NetSocket {
    pub fn new(stream: TcpStream, id: u64) -> NetSocket {
        NetSocket {
            id,
            stream,
            buffer: [0; 512],
            len: 0,         // number of bytes at the start of the buffer that are valid
            msg_size: None, // size of the command we are currently reading
        }
    }

    pub fn write(&mut self, message: &[u8]) -> Result<(), std::io::Error> {
        let msg = serialize_usize((message.len() + CHECKSUM_BYTES) as usize);
        let checksum = self.checksum(message);
        self.stream.write_all(&msg)?;
        self.stream.write_all(message)?;
        self.stream.write_all(&serialize_usize(checksum))?;
        Ok(())
    }

    pub fn read(&mut self) -> Result<Vec<u8>, std::io::Error> {
        let mut out = Vec::new();

        loop {
            if self.msg_size.is_none() && self.len >= 8 {
                self.msg_size = Some(deserialize_usize(&self.buffer[..]) as usize);
                self.buffer.rotate_left(MSG_SIZE_BYTES);
                self.len -= MSG_SIZE_BYTES;
                out.reserve(self.msg_size.unwrap());
            }
            if let Some(msg_size) = self.msg_size {
                let msg = msg_size - out.len();
                let max = self.len.min(msg);
                out.extend(&self.buffer[..max]);

                // if we used all the bytes in the buffer, there are 0 bytes leftover to use in the next iteration,
                // so we don't need to rotate
                if self.len != msg {
                    self.buffer.rotate_left(max);
                }
                self.len -= max;

                if out.len() > msg_size {
                    panic!("out.len() > command_size");
                }

                if out.len() == msg_size {
                    self.msg_size = None;
                    let checksum = deserialize_usize(&out[msg_size - CHECKSUM_BYTES..]);
                    out.truncate(out.len() - CHECKSUM_BYTES);

                    if self.checksum(&out) != checksum {
                        return Err(std::io::Error::new(ErrorKind::Other, "Checksum failed"));
                    }
                    return Ok(out);
                }
            }
            let read = self.stream.read(&mut self.buffer[self.len..])?;
            if read == 0 {
                return Err(std::io::Error::new(ErrorKind::Other, "Connection closed"));
            }
            self.len += read;
        }
    }

    fn checksum(&self, message: &[u8]) -> usize {
        let mut checksum = 0usize;
        for (i, byte) in message.iter().enumerate() {
            checksum = checksum.wrapping_add((*byte as usize) + i);
        }
        checksum
    }

    pub fn disconnect(&self) {
        self.stream.shutdown(std::net::Shutdown::Both).unwrap();
    }
}

fn deserialize_usize(vec: &[u8]) -> usize {
    let mut out: usize = 0;
    for i in 0..8 {
        out |= (vec[i] as usize) << (i * 8);
    }
    out
}

fn serialize_usize(num: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(8);
    for i in 0..8 {
        out.push((num >> (i * 8)) as u8);
    }
    out
}
