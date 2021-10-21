use async_std::{
    channel::{unbounded, Receiver},
    task,
};
use gilrs::EventType;
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
            let modified = match gilrs.next_event() {
                Some(gilrs::Event {
                    id: _,
                    event: ty,
                    time: _,
                }) => event.update(ty),
                None => {
                    if !sync.1 {
                        #[cfg(windows)]
                        task::sleep(sleep).await;
                        #[cfg(unix)]
                        std::thread::sleep(sleep);
                        sleep = Duration::max(sleep + Duration::from_millis(1), max_sleep);
                    }
                    false
                }
            };
            if modified {
                sleep = Duration::from_millis(1);
            }
            let time = Instant::now();
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
    });
    receiver
}

impl Event {
    fn update(&mut self, ty: EventType) -> bool {
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

#[test]
fn try_it() {
    let events = spawn(100);
    task::block_on(async move {
        while let Ok((_, event)) = events.recv().await {
            println!("{:?}", event);
        }
        println!("!");
    });
}
