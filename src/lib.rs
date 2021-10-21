use async_std::{
    channel::{unbounded, Receiver},
    task,
};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug)]
pub struct Event {
    pub gear: i8,
    pub speed: f32,
    pub direction: f32,
}

pub fn spawn(frequency: u16) -> Receiver<(Instant, Event)> {
    let (sender, receiver) = unbounded();
    task::spawn(async move {
        let max_sleep = Duration::from_secs(1) / frequency as u32;
        let mut sleep = Duration::from_millis(1);
        let mut sync = (Instant::now(), false);
        let mut event = Event {
            gear: 1,
            speed: 0.0,
            direction: 0.0,
        };
        let mut gilrs = gilrs::Gilrs::new().unwrap();
        loop {
            match gilrs.next_event() {
                Some(gilrs::Event {
                    id: _,
                    event: ty,
                    time: _,
                }) => {
                    let time = Instant::now();
                    use gilrs::{
                        Axis::LeftStickX,
                        Button::{North, RightTrigger2, South, West},
                        EventType::*,
                    };
                    let modified = match ty {
                        Disconnected => {
                            event.speed = 0.0;
                            event.direction = 0.0;
                            true
                        }
                        AxisChanged(LeftStickX, value, _) => {
                            event.direction = value;
                            true
                        }
                        ButtonChanged(RightTrigger2, value, _) => {
                            event.speed = value;
                            true
                        }
                        ButtonPressed(North, _) => match event.gear {
                            1..=5 => {
                                event.gear = -1;
                                true
                            }
                            _ => false,
                        },
                        ButtonReleased(North, _) => match event.gear {
                            -2 | -1 => {
                                event.gear = 1;
                                true
                            }
                            _ => false,
                        },
                        ButtonReleased(West, _) => event.gear_up(),
                        ButtonReleased(South, _) => event.gear_down(),
                        _ => false,
                    };
                    if modified {
                        sleep = Duration::from_millis(1);
                    }
                    if (modified || sync.1) && time >= sync.0 + max_sleep {
                        sync = (time, false);
                        #[cfg(windows)]
                        let _ = sender.send((time, event)).await;
                        #[cfg(unix)]
                        let _ = task::block_on(async { sender.send((time, event)).await });
                    } else {
                        sync.1 |= modified;
                    }
                }
                None => {
                    #[cfg(windows)]
                    task::sleep(sleep).await;
                    #[cfg(unix)]
                    std::thread::sleep(sleep);
                    sleep = Duration::max(sleep + Duration::from_millis(1), max_sleep);
                }
            }
        }
    });
    receiver
}

impl Event {
    fn gear_up(&mut self) -> bool {
        self.gear = match self.gear {
            -1 => -2,
            1 => 2,
            2 => 3,
            3 => 4,
            4 => 5,
            _ => return false,
        };
        true
    }

    fn gear_down(&mut self) -> bool {
        self.gear = match self.gear {
            -2 => -1,
            2 => 1,
            3 => 2,
            4 => 3,
            5 => 4,
            _ => return false,
        };
        true
    }
}
