use gilrs::EventType;

impl super::Event {
    pub fn update(&mut self, ty: EventType) -> bool {
        use gilrs::{
            Axis::LeftStickX,
            Button::{North, South, West},
            EventType::*,
        };
        match ty {
            Disconnected => {
                self.speed = 0.0;
                self.direction = 0.0;
                true
            }
            AxisChanged(LeftStickX, value, _) => {
                self.direction = value;
                true
            }
            #[cfg(windows)]
            ButtonChanged(gilrs::Button::RightTrigger2, value, _) => {
                self.speed = value;
                true
            }
            #[cfg(unix)]
            AxisChanged(gilrs::Axis::RightZ, value, _) => {
                self.speed = (value + 1.0) / 2.0;
                true
            }
            ButtonPressed(North, _) => match self.gear {
                1..=5 => {
                    self.gear = -1;
                    true
                }
                _ => false,
            },
            ButtonReleased(North, _) => match self.gear {
                -2 | -1 => {
                    self.gear = 1;
                    true
                }
                _ => false,
            },
            ButtonReleased(West, _) => self.gear_up(),
            ButtonReleased(South, _) => self.gear_down(),
            _ => {
                #[cfg(test)]
                println!("unspecific: {:?}", ty);
                false
            }
        }
    }
}
