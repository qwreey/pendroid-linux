use bytebuffer::ByteReader;
use qwreey_utility_rs::ErrToString;

#[allow(unused)]
pub struct Stylus {
    pub down: bool,
    pub button: bool,
    pub hover: bool,
    pub pressure: i16,
    pub tilt_x: i16,
    pub tilt_y: i16,
    pub x: i16,
    pub y: i16,
    pub timestamp: i32,
}

impl Stylus {
    pub fn new(buf: &mut ByteReader) -> Result<Self, String> {
        let flags = buf.read_u8().err_to_string()?;
        Ok(Stylus {
            down: flags & 0b0000_0001 != 0,
            button: flags & 0b0000_0010 != 0,
            hover: flags & 0b0000_0100 != 0,
            pressure: buf.read_i16().err_to_string()?,
            tilt_x: buf.read_i16().err_to_string()?,
            tilt_y: buf.read_i16().err_to_string()?,
            x: buf.read_i16().err_to_string()?,
            y: buf.read_i16().err_to_string()?,
            timestamp: buf.read_i32().err_to_string()?,
        })
    }
}
