use std::sync::Mutex;

fn main() {
    let res = Mutex::new(3);
    
    {
        println!("getting lock on res");
        let mut num = res.lock().unwrap(); // get lock on mutex
        *num = 6;
    }

    println!("res: {}", res.lock().unwrap());
}