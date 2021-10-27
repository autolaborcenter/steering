use super::{Context, Status, Steering};
use gilrs::{Axis, Button};

pub struct G29(Context);

impl Steering for G29 {
    fn new() -> Self {
        Self(Context::new())
    }

    fn status(&mut self) -> Status {
        use gilrs::EventType::*;
        while let Some(e) = self.0.handle_events() {
            if let ButtonReleased(Button::Unknown, code) = e {
                match code.into_u32() {
                    65827 => match self.0.level {
                        1.. => self.0.level = -1,
                        _ => self.0.level = 1,
                    },
                    65828 => self.0.gear_up(),
                    65829 => self.0.gear_down(),
                    _ => {}
                }
            }
        }
        let (rho, theta) = self.0.active.map_or((0.0, 0.0), |id| {
            let gampad = self.0.gilrs.gamepad(id);
            (gampad.value(Axis::LeftZ), gampad.value(Axis::LeftStickX))
        });
        Status {
            level: self.0.level,
            rho,
            theta,
        }
    }
}
