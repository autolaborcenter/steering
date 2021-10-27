use super::{Context, Status, Steering};
use gilrs::{Axis, Button};

pub struct Hori(Context, f32);

impl Steering for Hori {
    fn new() -> Self {
        Self(Context::new(), 0.0)
    }

    fn status(&mut self) -> Status {
        use gilrs::EventType::*;
        while let Some(e) = self.0.handle_events() {
            match e {
                ButtonReleased(Button::West, _) => self.0.gear_up(),
                ButtonReleased(Button::South, _) => self.0.gear_down(),
                ButtonReleased(Button::North, _) => match self.0.level {
                    1.. => self.0.level = -1,
                    _ => self.0.level = 1,
                },
                #[cfg(windows)]
                ButtonChanged(Button::RightTrigger2, value, _) => {
                    self.1 = value;
                }
                #[cfg(unix)]
                ButtonChanged(Axis::RightZ, value, _) => {
                    self.1 = value;
                }

                _ => {}
            };
        }
        let (y, x) = self.0.active.map_or((0.0, 0.0), |id| {
            (self.1, self.0.gilrs.gamepad(id).value(Axis::LeftStickX))
        });
        Status {
            level: self.0.level,
            x,
            y,
        }
    }
}
