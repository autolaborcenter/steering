use async_std::task;
use steering::spawn;

fn main() {
    let events = spawn();
    task::block_on(async move {
        while let Ok((_, event)) = events.recv().await {
            println!("{:?}", event);
        }
        println!("!");
    });
}
