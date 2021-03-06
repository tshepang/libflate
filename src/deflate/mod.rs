//! The encoder and decoder of the DEFLATE format and algorithm.
//!
//! The DEFLATE is defined in [RFC-1951](https://tools.ietf.org/html/rfc1951).
//!
//! # Examples
//! ```
//! use std::io::{self, Read};
//! use libflate::deflate::{Encoder, Decoder};
//!
//! // Encoding
//! let mut encoder = Encoder::new(Vec::new());
//! io::copy(&mut &b"Hello World!"[..], &mut encoder).unwrap();
//! let encoded_data = encoder.finish().into_result().unwrap();
//!
//! // Decoding
//! let mut decoder = Decoder::new(io::Cursor::new(encoded_data));
//! let mut decoded_data = Vec::new();
//! decoder.read_to_end(&mut decoded_data).unwrap();
//!
//! assert_eq!(decoded_data, b"Hello World!");
//! ```
pub use self::decode::Decoder;
pub use self::encode::Encoder;
pub use self::encode::EncodeOptions;
pub use self::encode::DEFAULT_BLOCK_SIZE;

mod decode;
mod encode;
mod symbol;

#[derive(Debug, Clone, Copy)]
enum BlockType {
    Raw = 0b00,
    Fixed = 0b01,
    Dynamic = 0b10,
}
