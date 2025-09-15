use bytebuffer::ByteReader;
use qwreey_utility_rs::ErrToString;

#[allow(unused)]
pub struct Finger {
    pub slot: u8,
    pub down: bool,
    pub total_down: u8,
    pub tracking_id: i32,
    pub x: i16,
    pub y: i16,
}

impl Finger {
    pub fn new(buf: &mut ByteReader) -> Result<Self, String> {
        Ok(Finger {
            slot: buf.read_u8().err_to_string()?,
            down: buf.read_u8().err_to_string()? != 0,
            total_down: buf.read_u8().err_to_string()?,
            tracking_id: buf.read_i32().err_to_string()?,
            x: buf.read_i16().err_to_string()?,
            y: buf.read_i16().err_to_string()?,
        })
    }
}
