use gilrs::{
    ff::{BaseEffect, BaseEffectType, EffectBuilder, Replay, Ticks},
    Axis::{LeftStickX, LeftZ},
    Button::Unknown,
    EventType::{self, *},
    GamepadId, Gilrs,
};

impl super::Event {
    pub fn update(&mut self, ty: EventType) -> bool {
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
            AxisChanged(LeftZ, value, _) => {
                self.speed = (1.0 - value) / 2.0;
                true
            }
            ButtonReleased(Unknown, code) => match code.into_u32() {
                65827 => {
                    self.gear = match self.gear {
                        1..=5 => -1,
                        _ => 1,
                    };
                    true
                }
                65828 => {
                    self.gear_up();
                    true
                }
                65829 => {
                    self.gear_down();
                    true
                }
                _n => {
                    #[cfg(test)]
                    println!("{}", _n);
                    false
                }
            },
            _ => {
                #[cfg(test)]
                println!("unspecific: {:?}", ty);
                false
            }
        }
    }
}
