use gilrs::{EventType, GamepadId, Gilrs};

#[cfg(feature = "hori")]
mod hori;

#[cfg(feature = "hori")]
pub type Device = hori::Hori;

#[cfg(feature = "xbox360")]
mod xbox360;

#[cfg(feature = "xbox360")]
pub type Device = xbox360::XBox360;

#[cfg(all(unix, feature = "g29"))]
mod g29;

#[cfg(all(unix, feature = "g29"))]
pub type Device = g29::G29;

#[derive(Clone, Copy, Debug)]
pub struct Status {
    pub level: i8,
    pub rho: f32,
    pub theta: f32,
}

pub trait Steering {
    fn new() -> Self;
    fn status(&mut self) -> Status;
}

struct Context {
    gilrs: Gilrs,
    active: Option<GamepadId>,
    level: i8,
}

impl Context {
    fn new() -> Self {
        Self {
            gilrs: Gilrs::new().unwrap(),
            active: None,
            level: 1,
        }
    }

    fn handle_events(&mut self) -> Option<EventType> {
        while let Some(e) = self.gilrs.next_event() {
            use gilrs::ev::EventType::*;

            if e.event == Disconnected {
                if Some(e.id) == self.active {
                    self.active = None;
                }
            } else {
                self.active = Some(e.id);
                return Some(e.event);
            }
        }
        return None;
    }

    fn gear_up(&mut self) {
        self.level = match self.level {
            -1 => -2,
            1 => 2,
            2 => 3,
            3 => 4,
            4 => 5,
            _ => self.level,
        };
    }

    fn gear_down(&mut self) {
        self.level = match self.level {
            -2 => -1,
            2 => 1,
            3 => 2,
            4 => 3,
            5 => 4,
            _ => self.level,
        };
    }
}

#[test]
fn try_it() {
    use std::{thread, time::Duration};

    let mut device = Device::new();
    loop {
        println!("{:?}", device.status());
        thread::sleep(Duration::from_millis(50));
    }
}
