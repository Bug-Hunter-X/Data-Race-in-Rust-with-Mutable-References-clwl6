use std::sync::{Arc, Mutex};

fn main() {
    let x = Arc::new(Mutex::new(5));
    let y = x.clone();
    let z = x.clone();

    let mut y_guard = y.lock().unwrap(); // Acquire lock
    *y_guard = 10;
    drop(y_guard); // Release lock

    let mut z_guard = z.lock().unwrap(); // Acquire lock
    *z_guard = 15;
    drop(z_guard); // Release lock

    println!("x = {}", *x.lock().unwrap());
} 