use bytebuffer::ByteReader;
use qwreey_utility_rs::ErrToString;

#[allow(unused)]
pub struct Init {
    pub width: u16,
    pub height: u16,
}

impl Init {
    pub fn new(buf: &mut ByteReader) -> Result<Self, String> {
        Ok(Init {
            width: buf.read_u16().err_to_string()?,
            height: buf.read_u16().err_to_string()?,
        })
    }
}
