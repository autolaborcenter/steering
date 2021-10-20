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

pub fn spawn() -> Receiver<(Instant, Event)> {
    let (sender, receiver) = unbounded();
    task::spawn(async move {
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
                    Disconnected => {
                        event.speed = 0.0;
                        event.direction = 0.0;
                        let _ = sender.send((time, event)).await;
                    }
                    AxisChanged(LeftStickX, value, _) => {
                        event.direction = value;
                        let _ = sender.send((time, event)).await;
                    }
                    ButtonChanged(RightTrigger2, value, _) => {
                        event.speed = value;
                        let _ = sender.send((time, event)).await;
                    }
                    ButtonPressed(North, _) => {
                        match event.gear {
                            1..=5 => {
                                event.gear = -1;
                                let _ = sender.send((time, event)).await;
                            }
                            _ => {}
                        };
                    }
                    ButtonReleased(North, _) => {
                        match event.gear {
                            -2 | -1 => {
                                event.gear = 1;
                                let _ = sender.send((time, event)).await;
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
                            let _ = sender.send((time, event)).await;
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
                            let _ = sender.send((time, event)).await;
                        }
                    }
                    _ => {}
                }
            }
            task::sleep(Duration::from_millis(1)).await;
        }
    });
    receiver
}
