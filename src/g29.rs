use super::{Context, Status, Steering};
use gilrs::{Axis, Button};
use nix::{
    fcntl::{self, OFlag},
    sys::stat::Mode,
};
use std::{fs, os::unix::prelude::RawFd};

pub struct G29 {
    context: Context,
    rho_initialized: bool,
}

impl Steering for G29 {
    fn new() -> Self {
        G29EventFd::find().map(|fd| fd.set_autocenter(0x6000));
        Self {
            context: Context::new(),
            rho_initialized: false,
        }
    }

    fn status(&mut self) -> Status {
        use gilrs::EventType::*;
        while let Some(e) = self.context.handle_events() {
            match e {
                ButtonReleased(Button::Unknown, code) => match code.into_u32() {
                    65827 => match self.context.level {
                        1.. => self.context.level = -1,
                        _ => self.context.level = 1,
                    },
                    65828 => self.context.gear_up(),
                    65829 => self.context.gear_down(),
                    _ => {}
                },
                AxisChanged(Axis::LeftZ, _, _) => {
                    self.rho_initialized = true;
                }
                Connected => {
                    self.rho_initialized = false;
                    G29EventFd::find().map(|fd| fd.set_autocenter(0x6000));
                }
                Disconnected => {
                    self.rho_initialized = false;
                }
                _ => {}
            }
        }
        let (rho, theta) = self.context.active.map_or((0.0, 0.0), |id| {
            let gampad = self.context.gilrs.gamepad(id);
            (
                if self.rho_initialized {
                    (1.0 - gampad.value(Axis::LeftZ)) / 2.0
                } else {
                    0.0
                },
                gampad.value(Axis::LeftStickX),
            )
        });
        Status {
            level: self.context.level,
            rho,
            theta,
        }
    }
}

struct G29EventFd(RawFd);

#[repr(C)]
struct InputEvent {
    time: [u64; 2],
    type_: u16,
    code: u16,
    value: i32,
}

impl G29EventFd {
    fn find() -> Option<Self> {
        const FILE: &str = "/proc/bus/input/devices";
        const NAME: &str = "N: Name=\"Logitech G29 Driving Force Racing Wheel\"";
        const HANDLE: &str = "H: Handlers=";

        let file = fs::read_to_string(FILE).unwrap();
        let lines = file.lines().collect::<Vec<_>>();
        lines
            .split_inclusive(|line| line.is_empty())
            .find(|b| b.len() >= 4 && b[1] == NAME)
            .and_then(|lines| lines.iter().find(|l| l.starts_with(HANDLE)))
            .and_then(|l| {
                l[HANDLE.len()..]
                    .split_whitespace()
                    .find(|it| it.starts_with("event"))
            })
            .and_then(|e| e[5..].parse::<u8>().ok())
            .and_then(|n| {
                fcntl::open(
                    format!("/dev/input/event{}", n).as_str(),
                    OFlag::O_RDWR,
                    Mode::empty(),
                )
                .ok()
            })
            .map(|fd| Self(fd))
    }

    fn set_autocenter(&self, value: i32) {
        const EV_FF: u16 = 0x15;
        const FF_AUTOCENTER: u16 = 0x61;

        let e = InputEvent {
            time: [0, 0],
            type_: EV_FF,
            code: FF_AUTOCENTER,
            value,
        };
        let buf = unsafe {
            std::slice::from_raw_parts(
                &e as *const _ as *const u8,
                std::mem::size_of::<InputEvent>(),
            )
        };
        let _ = nix::unistd::write(self.0, buf);
    }
}

impl Drop for G29EventFd {
    fn drop(&mut self) {
        let _ = nix::unistd::close(self.0);
    }
}

#[test]
fn test_find_device() {
    G29EventFd::find().map(|fd| fd.set_autocenter(0x6000));
}
