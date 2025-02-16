//! bytes utils

pub trait Bytes {
    fn to_hex(&self) -> String;
}

impl Bytes for &[u8] {
    fn to_hex(&self) -> String {
        self.iter().fold(String::new(), |mut acc, byte| {
            acc.push_str(&format!("{:02x}", byte));
            acc
        })
    }
}
