mod event_list;
mod finger;
mod stylus;
mod with_abs;

use super::super::parse::{Event, Init};
use finger::FingerBackend;
use stylus::StylusBackend;

pub struct InputBackend {
    stylus: StylusBackend,
    finger: FingerBackend,
}
impl InputBackend {
    pub fn new(init_data: &Init) -> Result<Self, String> {
        Ok(Self {
            stylus: StylusBackend::new(init_data)?,
            finger: FingerBackend::new(init_data)?,
        })
    }

    pub fn execute(&mut self, event: Event) -> Result<(), String> {
        match event {
            Event::Finger(finger_data) => self.finger.process(&finger_data),
            Event::Stylus(stylus_data) => self.stylus.process(&stylus_data),
            Event::Init(_) => Ok(()),
        }
    }
}
