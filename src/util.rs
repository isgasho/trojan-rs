use tokio::io::{self, AsyncRead, AsyncWrite};

use crate::error::Result;

pub fn sha224(data: &[u8]) -> [u8; 28] {
    openssl::sha::sha224(data)
}

pub trait ToHex {
    fn to_hex(&self) -> String;
}

impl ToHex for [u8] {
    fn to_hex(&self) -> String {
        const CHARS: &[u8] = b"0123456789abcdef";

        let mut v = Vec::with_capacity(self.len() * 2);
        for &b in self {
            v.push(CHARS[(b >> 4) as usize]);
            v.push(CHARS[(b & 0xf) as usize]);
        }

        unsafe { String::from_utf8_unchecked(v) }
    }
}

pub trait TrimInPlace {
    fn trim_in_place(self: &mut Self);
}

impl TrimInPlace for String {
    fn trim_in_place(self: &mut Self) {
        let trimmed = self.trim();
        let len = trimmed.len();

        unsafe {
            core::ptr::copy(trimmed.as_ptr(), self.as_mut_ptr(), len);
        }
        self.truncate(len);
    }
}

pub async fn link_stream<A: AsyncRead + AsyncWrite, B: AsyncRead + AsyncWrite>(
    a: A,
    b: B,
) -> Result<()> {
    let (ar, aw) = &mut io::split(a);
    let (br, bw) = &mut io::split(b);

    let r = tokio::select! {
        r1 = io::copy(ar, bw) => {
            r1
        },
        r2 = io::copy(br, aw) => {
            r2
        }
    };

    Ok(r.map(drop)?)
}
