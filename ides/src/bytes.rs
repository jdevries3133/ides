//! bytes utils

use base64::prelude::*;

pub trait Bytes {
    fn to_hex(&self) -> String;
    fn to_base64(&self) -> String;
}

impl Bytes for [u8] {
    fn to_hex(&self) -> String {
        self.iter().fold(String::new(), |mut acc, byte| {
            acc.push_str(&format!("{:02x}", byte));
            acc
        })
    }
    fn to_base64(&self) -> String {
        BASE64_STANDARD.encode(self)
    }
}
