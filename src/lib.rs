use async_std::{
    channel::{unbounded, Receiver},
    task,
};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug)]
pub struct Event {
    gear: i8,
    speed: f32,
    direction: f32,
}

macro_rules! send_blocking {
    ($msg:expr => $sender:expr) => {{
        let _ = task::block_on(async { $sender.send($msg).await });
    }};
}

pub fn spawn() -> Receiver<(Instant, Event)> {
    let (sender, receiver) = unbounded();
    task::spawn_blocking(move || {
        let mut event = Event {
            gear: 1,
            speed: 0.0,
            direction: 0.0,
        };
        let mut gilrs = gilrs::Gilrs::new().unwrap();
        loop {
            while let Some(gilrs::Event {
                id: _,
                event: ty,
                time: _,
            }) = gilrs.next_event()
            {
                let time = Instant::now();
                use gilrs::{
                    Axis::LeftStickX,
                    Button::{North, RightTrigger2, South, West},
                    EventType::*,
                };
                match ty {
                    AxisChanged(LeftStickX, value, _) => {
                        event.direction = value;
                        send_blocking!((time, event) => sender);
                    }
                    ButtonChanged(RightTrigger2, value, _) => {
                        event.speed = value;
                        send_blocking!((time, event) => sender);
                    }
                    ButtonPressed(North, _) => {
                        match event.gear {
                            1..=5 => {
                                event.gear = -1;
                                send_blocking!((time, event) => sender);
                            }
                            _ => {}
                        };
                    }
                    ButtonReleased(North, _) => {
                        match event.gear {
                            -2 | -1 => {
                                event.gear = 1;
                                send_blocking!((time, event) => sender);
                            }
                            _ => {}
                        };
                    }
                    ButtonReleased(West, _) => {
                        let next = match event.gear {
                            -1 => -2,
                            1 => 2,
                            2 => 3,
                            3 => 4,
                            4 => 5,
                            _ => continue,
                        };
                        if next != event.gear {
                            event.gear = next;
                            send_blocking!((time, event) => sender);
                        }
                    }
                    ButtonReleased(South, _) => {
                        let next = match event.gear {
                            -2 => -1,
                            2 => 1,
                            3 => 2,
                            4 => 3,
                            5 => 4,
                            _ => continue,
                        };
                        if next != event.gear {
                            event.gear = next;
                            send_blocking!((time, event) => sender);
                        }
                    }
                    _ => {}
                }
            }
            task::block_on(async { task::sleep(Duration::from_millis(1)).await });
        }
    });
    receiver
}
