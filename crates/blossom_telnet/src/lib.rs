/// This crate is is based off of https://github.com/jtenner/telnet_codec by jtenner, which is
/// more complete, as far as the Telnet protocol is concerned - but is also archived.
pub mod constants;
pub mod event;
pub mod option;
pub mod subnegotiation;

use std::mem;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use event::TelnetEvent;
use option::TelnetOption;
use subnegotiation::SubnegotiationType;
use thiserror::Error;
use tokio_util::codec::{Decoder, Encoder};

use crate::constants::*;

type Result<T> = std::result::Result<T, TelnetCodecError>;

#[derive(Debug, Error)]
pub enum TelnetCodecError {
    #[error("codec error: {0}")]
    CodecError(String),
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
}

/// Implements a Tokio codec for the Telnet protocol, along with MUD-specific extension protocols
/// such as GMCP. You should never have to interact with this directly.
#[derive(Debug)]
pub struct TelnetCodec {
    pub sga: bool,
    max_buffer_length: usize,
    buffer: Vec<u8>,
}

impl TelnetCodec {
    pub fn new(max_buffer_length: usize) -> Self {
        TelnetCodec {
            sga: false,
            max_buffer_length,
            buffer: Vec::new(),
        }
    }
}

impl Decoder for TelnetCodec {
    type Item = TelnetEvent;
    type Error = TelnetCodecError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>> {
        let x = 0;

        if self.sga && !self.buffer.is_empty() {
            let buf = mem::take(&mut self.buffer);
            let result = String::from_utf8_lossy(&buf[..]);

            return Ok(Some(TelnetEvent::Message(result.to_string())));
        }

        if src.is_empty() {
            return Ok(None);
        }

        if self.sga {
            return decode_sga(x, src);
        }

        decode_loop(self, x, src)
    }
}

impl Encoder<TelnetEvent> for TelnetCodec {
    type Error = TelnetCodecError;

    fn encode(&mut self, event: TelnetEvent, buf: &mut BytesMut) -> Result<()> {
        match event {
            TelnetEvent::Do(option) => encode_negotiate(DO, option, buf),
            TelnetEvent::Dont(option) => encode_negotiate(DONT, option, buf),
            TelnetEvent::Will(option) => encode_negotiate(WILL, option, buf),
            TelnetEvent::Wont(option) => encode_negotiate(WONT, option, buf),
            TelnetEvent::Subnegotiation(sb_type) => encode_sb(sb_type, buf),
            TelnetEvent::Message(msg) => encode_message(msg, buf),
            _ => {}
        }

        Ok(())
    }
}

fn decode_negotiate(x: usize, src: &mut BytesMut, opt: u8) -> Result<Option<TelnetEvent>> {
    if x + 2 >= src.len() {
        return Ok(None);
    }

    let option = src[x + 2];
    src.advance(x + 3);
    match opt {
        WILL => Ok(Some(TelnetEvent::Will(option.into()))),
        WONT => Ok(Some(TelnetEvent::Wont(option.into()))),
        DO => Ok(Some(TelnetEvent::Do(option.into()))),
        DONT => Ok(Some(TelnetEvent::Dont(option.into()))),
        _ => Ok(None),
    }
}

fn decode_sga(x: usize, src: &mut BytesMut) -> Result<Option<TelnetEvent>> {
    match src[0] {
        IAC => {
            if 1 >= src.len() {
                return Ok(None);
            }

            match src[x + 1] {
                IAC => {
                    src.advance(2);
                    Ok(Some(TelnetEvent::Character(IAC)))
                }
                _ => Ok(None),
            }
        }
        _ => Ok(None),
    }
}

fn decode_naws(subvec: Vec<u8>) -> Result<Option<TelnetEvent>> {
    match subvec.len() {
        4 => {
            let result = SubnegotiationType::WindowSize(
                ((subvec[0] as u16) << 8) | (subvec[1] as u16),
                ((subvec[2] as u16) << 8) | (subvec[3] as u16),
            );
            Ok(Some(TelnetEvent::Subnegotiation(result)))
        }
        _ => Ok(None),
    }
}

fn decode_unknown(opt: u8, subvec: Vec<u8>) -> Result<Option<TelnetEvent>> {
    let result = SubnegotiationType::Unknown(opt.into(), Bytes::from(subvec));
    Ok(Some(TelnetEvent::Subnegotiation(result)))
}

fn decode_next_byte(codec: &mut TelnetCodec, buf_len: &mut usize, byte: u8) {
    if buf_len < &mut codec.max_buffer_length {
        codec.buffer.push(byte);
        *buf_len += 1;
    }
}

fn decode_se(
    invalid: bool,
    src: &mut BytesMut,
    subvec: Vec<u8>,
    opt: u8,
) -> Result<Option<TelnetEvent>> {
    src.split_at(2);

    if invalid {
        Ok(None)
    } else {
        match opt {
            NAWS => decode_naws(subvec),
            _ => decode_unknown(opt, subvec),
        }
    }
}

fn decode_loop(
    codec: &mut TelnetCodec,
    x: usize,
    src: &mut BytesMut,
) -> Result<Option<TelnetEvent>> {
    let mut x = x;
    let mut buf_len = codec.buffer.len();

    loop {
        if x >= src.len() {
            return Ok(None);
        }

        // Handle matches against the first byte in the buffer.
        match src[x] {
            IAC => {
                if x + 1 >= src.len() {
                    return Ok(None);
                }

                // Handle matches against the second byte in the buffer.
                match src[x + 1] {
                    IAC => {
                        if codec.buffer.len() < codec.max_buffer_length {
                            codec.buffer.push(IAC);
                            buf_len += 1;
                        }

                        x += 1;
                    }
                    DO => return decode_negotiate(x, src, DO),
                    DONT => return decode_negotiate(x, src, DONT),
                    WILL => return decode_negotiate(x, src, WILL),
                    WONT => return decode_negotiate(x, src, WONT),
                    SB => {
                        if x + 2 >= src.len() {
                            src.advance(x + 2);
                            return Ok(None);
                        }

                        let start = x;
                        let opt = src[x + 2];

                        x += 3;

                        let mut subvec: Vec<u8> = Vec::new();
                        let mut invalid = false;

                        loop {
                            if x > src.len() {
                                src.advance(start);
                                return Ok(None);
                            }

                            // Handle matches against the third byte in the buffer.
                            // This is for subnegotiation.
                            match src[x] {
                                IAC => {
                                    if x + 1 > src.len() {
                                        return Ok(None);
                                    }

                                    // Handle matches against the fourth byte in the buffer.
                                    // This is the final byte in the buffer.
                                    match src[x + 1] {
                                        SE => return decode_se(invalid, src, subvec, opt),
                                        IAC => subvec.push(IAC),
                                        _ => invalid = true,
                                    }

                                    x += 1;
                                }
                                _ => subvec.push(src[x]),
                            }

                            x += 1;
                        }
                    }
                    NOP => x += 1,
                    _ => {}
                }
            }
            b'\n' => {
                let mut buffer = mem::take(&mut codec.buffer);
                if buffer.ends_with(&[b'\r']) {
                    buffer.pop();
                    src.advance(x + 1);

                    let result = String::from_utf8_lossy(&buffer[..]);
                    return Ok(Some(TelnetEvent::Message(result.to_string())));
                }

                decode_next_byte(codec, &mut buf_len, src[x]);
            }
            _ => decode_next_byte(codec, &mut buf_len, src[x]),
        };

        x += 1;
    }
}

fn encode_negotiate(opt: u8, subopt: TelnetOption, buf: &mut BytesMut) {
    buf.reserve(3);
    buf.put_u8(IAC);

    match opt {
        DO => buf.put_u8(DO),
        DONT => buf.put_u8(DONT),
        WILL => buf.put_u8(WILL),
        WONT => buf.put_u8(WONT),
        _ => unreachable!(),
    }

    buf.put_u8(subopt.into());
}

fn encode_sb(sb: SubnegotiationType, buf: &mut BytesMut) {
    match sb {
        SubnegotiationType::WindowSize(width, height) => {
            buf.reserve(9);
            buf.extend(&[IAC, SB, NAWS]);
            buf.put_u16(width);
            buf.put_u16(height);
            buf.extend(&[IAC, SE]);
        }
        SubnegotiationType::Unknown(option, bytes) => {
            let mut len = bytes.len() + 5;

            for x in &bytes {
                if *x == IAC {
                    len += 1;
                }
            }

            buf.reserve(len);

            // IAC SUB OPTION
            buf.extend(&[IAC, SB, option.into()]);

            // Write to the buffer
            for byte in &bytes {
                if *byte == IAC {
                    buf.extend(&[IAC, IAC]);
                } else {
                    buf.put_u8(*byte);
                }
            }

            // IAC SUBNEGOTIATION END
            buf.extend(&[IAC, SE]);
        }
    }
}

fn encode_message(msg: String, buf: &mut BytesMut) {
    let bytes = Bytes::from(msg);
    let mut len = bytes.len();
    for x in &bytes {
        if *x == IAC {
            len += 1;
        }
    }

    buf.reserve(len);

    for x in &bytes {
        if *x == IAC {
            buf.extend(&[IAC, IAC]);
        }
        buf.put_u8(*x);
    }

    if !buf.ends_with(b"\r\n") {
        buf.reserve(2);
        buf.extend(&[b'\r', b'\n']);
    }
}
