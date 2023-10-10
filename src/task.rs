use std::time::Duration;
use async_std::task;

async fn hello() -> String { // <-- asyncronous fun, return type is a future
    task::sleep(Duration::from_secs(2)).await;
    String::from("Hello")
}

async fn world() -> String {
    String::from(" World!")
}

async fn async_main() -> String {
    println!("Started!");
    let hello_future = hello();
    let world_future = world();
    let hello_result = hello_future.await; // <-- wait for the future to complete
    let world_result = world_future.await; // <-- wait for the future to complete
    hello_result + world_result.as_str()
}

fn main() {
    // spawns an asynchronous task and blocks the current thread until it completes
    println!("{}", task::block_on(async_main()));
}