use super::{
    super::super::parse::{Finger, Init},
    event_list::PushEvent,
    with_abs::WithAbs,
};

use evdev::{
    AbsInfo, AbsoluteAxisCode, AttributeSet, BusType, InputEvent, InputId, KeyCode, PropType,
    UinputAbsSetup, uinput::VirtualDevice,
};
use qwreey_utility_rs::ErrToString;

const ABS_MT_SLOT: u16 = AbsoluteAxisCode::ABS_MT_SLOT.0;
const ABS_MT_POSITION_X: u16 = AbsoluteAxisCode::ABS_MT_POSITION_X.0;
const ABS_MT_POSITION_Y: u16 = AbsoluteAxisCode::ABS_MT_POSITION_Y.0;
const ABS_MT_TRACKING_ID: u16 = AbsoluteAxisCode::ABS_MT_TRACKING_ID.0;
const ABS_X: u16 = AbsoluteAxisCode::ABS_X.0;
const ABS_Y: u16 = AbsoluteAxisCode::ABS_Y.0;
const TOUCHS: [KeyCode; 5] = [
    KeyCode::BTN_TOOL_FINGER,
    KeyCode::BTN_TOOL_DOUBLETAP,
    KeyCode::BTN_TOOL_TRIPLETAP,
    KeyCode::BTN_TOOL_QUADTAP,
    KeyCode::BTN_TOOL_QUINTTAP,
];

pub struct FingerBackend {
    device: VirtualDevice,
    current_slot: i32,
    current_touching: bool,
    current_count: i32,
    inputs: Vec<InputEvent>,
    touch_trackings: [i32; 12],
    touch_active: [bool; 12],
}

impl FingerBackend {
    // Create new evdev device
    pub fn new(init_data: &Init) -> Result<Self, String> {
        let mut device = VirtualDevice::builder()
            .err_to_string()?
            .name("pendroid-touchpad")
            .input_id(InputId::new(BusType::BUS_USB, 0u16, 1333u16, 1u16))
            .with_abs(&[
                // TOOL INFO
                UinputAbsSetup::new(
                    AbsoluteAxisCode::ABS_MT_TOOL_TYPE,
                    AbsInfo::new(2, 0, 0, 0, 0, 1),
                ),
                // ABS X / Y
                UinputAbsSetup::new(
                    AbsoluteAxisCode::ABS_X,
                    AbsInfo::new(0, 0, init_data.width as i32, 6, 10, 11),
                ),
                UinputAbsSetup::new(
                    AbsoluteAxisCode::ABS_Y,
                    AbsInfo::new(0, 0, init_data.height as i32, 6, 10, 11),
                ),
                // ABS MT X / Y
                UinputAbsSetup::new(
                    AbsoluteAxisCode::ABS_MT_POSITION_X,
                    AbsInfo::new(0, 0, init_data.width as i32, 6, 10, 11),
                ),
                UinputAbsSetup::new(
                    AbsoluteAxisCode::ABS_MT_POSITION_Y,
                    AbsInfo::new(0, 0, init_data.height as i32, 6, 10, 11),
                ),
                // ABS SLOT
                UinputAbsSetup::new(
                    AbsoluteAxisCode::ABS_MT_SLOT,
                    AbsInfo::new(0, 0, 12, 0, 0, 1),
                ),
                // ABS_MT_TRACKING_ID
                UinputAbsSetup::new(
                    AbsoluteAxisCode::ABS_MT_TRACKING_ID,
                    AbsInfo::new(0, -1, 65535, 0, 0, 1),
                ),
            ])?
            .with_keys(&AttributeSet::from_iter([
                KeyCode::BTN_TOUCH,
                KeyCode::BTN_TOOL_FINGER,
                KeyCode::BTN_TOOL_DOUBLETAP,
                KeyCode::BTN_TOOL_TRIPLETAP,
                KeyCode::BTN_TOOL_QUADTAP,
                KeyCode::BTN_TOOL_QUINTTAP,
                KeyCode::BTN_LEFT,
            ]))
            .err_to_string()?
            .with_properties(&AttributeSet::from_iter([
                PropType::POINTER,
                PropType::BUTTONPAD,
            ]))
            .err_to_string()?
            .build()
            .err_to_string()?;

        for path in device.enumerate_dev_nodes_blocking().err_to_string()? {
            let path = path.err_to_string()?;
            tracing::info!("New finger backend available as {}", path.display());
        }

        Ok(Self {
            device,
            inputs: Vec::<InputEvent>::with_capacity(32),
            current_slot: -1,
            current_touching: false,
            current_count: 0,
            touch_active: [false; 12],
            touch_trackings: [-1i32; 12],
        })
    }

    // Update slot
    pub fn update_slot(&mut self, new_slot: u8) {
        let slot = new_slot as i32;
        if slot != self.current_slot {
            self.current_slot = slot;
            self.inputs.push_abs_event(ABS_MT_SLOT, slot);
        }
    }

    pub fn process(&mut self, touch_data: &Finger) -> Result<(), String> {
        self.inputs.clear();
        let index = touch_data.slot as usize;
        let x = touch_data.x as i32;
        let y = touch_data.y as i32;

        // Update ABS_MT_POSITION XY
        if touch_data.x != -1 {
            self.update_slot(touch_data.slot);
            self.inputs.push_abs_event(ABS_MT_POSITION_X, x);
            self.inputs.push_abs_event(ABS_X, x);
        }
        if touch_data.y != -1 {
            self.update_slot(touch_data.slot);
            self.inputs.push_abs_event(ABS_MT_POSITION_Y, y);
            self.inputs.push_abs_event(ABS_Y, y);
        }

        // Update ABS_MT_TRACKING_ID
        if self.touch_trackings[index] != touch_data.tracking_id {
            self.update_slot(touch_data.slot);
            self.touch_trackings[index] = touch_data.tracking_id;
            self.inputs
                .push_abs_event(ABS_MT_TRACKING_ID, touch_data.tracking_id);
        }

        // Count touch
        self.touch_active[index] = touch_data.down;
        let mut count = 0;
        for active in self.touch_active {
            if active {
                count += 1;
            }
        }

        // Change mode (Finger / Double / ...)
        if self.current_count != count {
            if self.current_count != 0 {
                self.inputs
                    .push_key(&TOUCHS[self.current_count as usize], 0);
            }
            if count != 0 {
                self.inputs.push_key(&TOUCHS[count as usize], 1);
            }
        }
        self.current_count = count;

        // Touch event (BTN_TOUCH)
        let touching = count != 0;
        if self.current_touching != touching {
            self.current_touching = touching;
            self.inputs.push_key(&KeyCode::BTN_TOUCH, touching as i32);
        }

        self.device.emit(self.inputs.as_slice()).err_to_string()?;
        Ok(())
    }
}
