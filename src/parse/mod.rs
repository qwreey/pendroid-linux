use bytebuffer::ByteReader;
use qwreey_utility_rs::ErrToString;

mod finger;
mod init;
mod stylus;

pub use finger::Finger;
pub use init::Init;
pub use stylus::Stylus;

pub enum Event {
    Init(Init),
    Stylus(Stylus),
    Finger(Finger),
}

impl Event {
    pub fn parse(buf: &mut ByteReader) -> Result<Event, String> {
        let event_type = buf.read_u8().err_to_string()?;

        Ok(match event_type {
            0x0 => Event::Init(Init::new(buf)?),
            0x1 => Event::Stylus(Stylus::new(buf)?),
            0x2 => Event::Finger(Finger::new(buf)?),
            _ => return Err(String::from("Got unexpected event type")),
        })
    }
}
