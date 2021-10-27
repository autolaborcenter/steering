use super::{Context, Status, Steering};
use gilrs::{Axis, Button, Gamepad};

pub struct XBox360(Context);

impl Steering for XBox360 {
    fn new() -> Self {
        Self(Context::new())
    }

    fn status(&mut self) -> Status {
        use gilrs::EventType::*;
        while let Some(e) = self.0.handle_events() {
            match e {
                ButtonReleased(Button::Start, _) => self.0.gear_up(),
                ButtonReleased(Button::Select, _) => self.0.gear_down(),
                _ => {}
            };
        }
        let (y, x) = self
            .0
            .active
            .map_or((0.0, 0.0), |id| map(&self.0.gilrs.gamepad(id)));
        Status {
            level: self.0.level,
            rho: f32::max(x.abs(), y.abs()),
            theta: x.atan2(y),
        }
    }
}

fn map(gamepad: &Gamepad) -> (f32, f32) {
    let buttons = (
        gamepad.is_pressed(Button::North),
        gamepad.is_pressed(Button::South),
        gamepad.is_pressed(Button::West),
        gamepad.is_pressed(Button::East),
        gamepad.is_pressed(Button::DPadUp),
        gamepad.is_pressed(Button::DPadDown),
        gamepad.is_pressed(Button::DPadLeft),
        gamepad.is_pressed(Button::DPadRight),
    );
    match buttons {
        (true, _, true, false, _, _, _, _) => button(1, 1),
        (true, _, false, true, _, _, _, _) => button(1, -1),
        (true, _, _, _, _, _, _, _) => button(1, 0),
        (_, true, true, false, _, _, _, _) => button(-1, 1),
        (_, true, false, true, _, _, _, _) => button(-1, -1),
        (_, true, _, _, _, _, _, _) => button(-1, 0),
        (_, _, true, false, _, _, _, _) => button(0, 1),
        (_, _, false, true, _, _, _, _) => button(0, -1),
        (_, _, true, true, _, _, _, _) => button(0, 0),
        (_, _, _, _, true, _, true, false) => button(1, 1),
        (_, _, _, _, true, _, false, true) => button(1, -1),
        (_, _, _, _, true, _, _, _) => button(1, 0),
        (_, _, _, _, _, true, true, false) => button(-1, 1),
        (_, _, _, _, _, true, false, true) => button(-1, -1),
        (_, _, _, _, _, true, _, _) => button(-1, 0),
        (_, _, _, _, _, _, true, false) => button(0, 1),
        (_, _, _, _, _, _, false, true) => button(0, -1),
        (_, _, _, _, _, _, true, true) => button(0, 0),
        _ => {
            let lx = gamepad.value(Axis::LeftStickY);
            let ly = gamepad.value(Axis::LeftStickY);
            let rx = gamepad.value(Axis::RightStickX);
            let ry = gamepad.value(Axis::RightStickY);
            let x = if lx.abs() > rx.abs() { lx } else { rx };
            let y = if ly.abs() > ry.abs() { ly } else { ry };
            (y, x)
        }
    }
}

#[inline]
fn button(y: i8, x: i8) -> (f32, f32) {
    (y as f32, x as f32)
}
