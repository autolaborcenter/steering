use std::{thread, time::Duration};
use steering::{Device, Steering};

fn main() {
    let mut device = Device::new();
    loop {
        println!("{:?}", device.status());
        thread::sleep(Duration::from_millis(50));
    }
}
