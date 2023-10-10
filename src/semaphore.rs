use std_semaphore::Semaphore;

fn main() {
    // Create a semaphore that represents 5 resources
    let sem = Semaphore::new(5);

    // Acquire one of the resources
    sem.acquire();

    {
        // Acquire one of the resources for a limited period of time
        let _guard = sem.access(); // <-- access = acquire + release
        // ...
    } // resources is released here

    // Release our initially acquired resource
    sem.release();
}
