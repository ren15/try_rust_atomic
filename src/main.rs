use std::sync::atomic::{AtomicBool, AtomicI32, Ordering::Relaxed};
use std::sync::Arc;

static X: AtomicI32 = AtomicI32::new(0);
static Y: AtomicI32 = AtomicI32::new(0);

fn taskset(cpuid: usize) {
    core_affinity::set_for_current(core_affinity::CoreId { id: cpuid });
}

fn a(running: Arc<AtomicBool>) -> u128 {
    taskset(1);
    let mut count = 0u128;
    while running.load(Relaxed) {
        count += 1;
        X.store(10, Relaxed);
        Y.store(20, Relaxed);
    }
    count
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

fn test_avg(x_a: i32, y_a: i32, iter_cnt: usize) {
    println!("trying: x: {}, y: {}", x_a, y_a);

    let mut a_count_v = vec![];
    let mut b_count_v = vec![];

    for _ in 0..iter_cnt {
        let running = Arc::new(AtomicBool::new(true));

        let a_th = {
            let running = running.clone();
            std::thread::spawn(move || a(running))
        };
        let b_th = std::thread::spawn(move || b(x_a, y_a));

        b_count_v.push(b_th.join().unwrap());
        running.store(false, Relaxed);
        a_count_v.push(a_th.join().unwrap());
    }

    let a_count_avg = a_count_v.iter().sum::<u128>() as f64 / a_count_v.len() as f64;
    let b_count_avg = b_count_v.iter().sum::<u128>() as f64 / b_count_v.len() as f64;
    println!("a thread joined at count: {}", a_count_avg);
    println!("b thread joined at count: {}", b_count_avg);

    println!("x: {}, y: {} happened from b's view", x_a, y_a);
    println!("--------------------------------");
}

fn main() {
    let iter_cnt = 1;
    test_avg(0, 0, iter_cnt);
    test_avg(10, 0, iter_cnt);
    test_avg(10, 20, iter_cnt);
    test_avg(0, 20, iter_cnt);
}
