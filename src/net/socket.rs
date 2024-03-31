use std::io::{Read, Write};
use std::net::TcpStream;

pub struct NetSocket {
    stream: TcpStream,
    buffer: [u8; 512],
    len: usize,
    command_size: Option<usize>,
}

impl NetSocket {
    pub fn new(stream: TcpStream) -> NetSocket {
        NetSocket {
            stream,
            buffer: [0; 512],
            len: 0,             // number of bytes at the start of the buffer that are valid
            command_size: None, // size of the command we are currently reading
        }
    }

    pub fn write(&mut self, message: &[u8]) -> Result<(), std::io::Error> {
        let message_size_bytes = serialize_u64(message.len() as u64);
        self.stream.write_all(&message_size_bytes)?;
        self.stream.write_all(message)?;
        Ok(())
    }

    pub fn read(&mut self) -> Result<Vec<u8>, std::io::Error> {
        let mut out = Vec::new();

        loop {
            if self.command_size.is_none() && self.len >= 8 {
                self.command_size = Some(deserialize_u64(&self.buffer[..]) as usize);
                self.buffer.rotate_left(8);
                self.len -= 8;
            }
            if let Some(command_size) = self.command_size {
                let command_size_leftover = command_size - out.len();
                let max = self.len.min(command_size_leftover);
                out.extend(&self.buffer[..max]);

                // if we used all the bytes in the buffer, there are 0 bytes leftover to use in the next iteration,
                // so we don't need to rotate
                if self.len != command_size_leftover {
                    self.buffer.rotate_left(max);
                }
                self.len -= max;

                if out.len() > command_size {
                    panic!("out.len() > command_size");
                }
                if out.len() == command_size {
                    self.command_size = None;
                    return Ok(out);
                }
            }
            let read = self.stream.read(&mut self.buffer[self.len..])?;
            if read == 0 {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Connection closed"));
            }
            self.len += read;
        }
    }
}

fn deserialize_u64(vec: &[u8]) -> u64 {
    let mut out: u64 = 0;
    for i in 0..8 {
        out |= (vec[i] as u64) << (i * 8);
    }
    out
}

fn serialize_u64(num: u64) -> Vec<u8> {
    let mut out = Vec::with_capacity(8);
    for i in 0..8 {
        out.push((num >> (i * 8)) as u8);
    }
    out
}
