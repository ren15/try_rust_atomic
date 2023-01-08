use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicBool, AtomicI32};
use std::sync::Arc;

static X: AtomicI32 = AtomicI32::new(0);
static Y: AtomicI32 = AtomicI32::new(0);

fn taskset(id: usize) {
    core_affinity::set_for_current(core_affinity::CoreId { id });
}

fn a(running: Arc<AtomicBool>) {
    taskset(1);
    while running.load(Relaxed) {
        X.store(10, Relaxed);
        Y.store(20, Relaxed);
    }
}

fn b(x_a: i32, y_a: i32) -> u128 {
    taskset(2);
    let mut count = 0u128;
    loop {
        count += 1;
        let y = Y.load(Relaxed);
        let x = X.load(Relaxed);
        X.store(0, Relaxed);
        Y.store(0, Relaxed);
        if x == x_a && y == y_a {
            return count;
        }
    }
}

fn test_i(x_a: i32, y_a: i32) {
    println!("trying: x: {}, y: {}", x_a, y_a);
    let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));

    let a_handle = {
        let running = running.clone();
        std::thread::spawn(move || a(running))
    };
    let b_handle = std::thread::spawn(move || b(x_a, y_a));
    let count = b_handle.join().unwrap();
    println!("b thread joined at count: {}", count);
    running.store(false, Relaxed);
    a_handle.join().unwrap();
    println!("a thread joined");
    println!("x: {}, y: {} happened", x_a, y_a);
    println!("--------------------------------");
}

fn main() {
    test_i(0, 0);
    test_i(10, 0);
    test_i(10, 20);
    test_i(0, 20);
}
