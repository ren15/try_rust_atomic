use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering::Relaxed};
use std::sync::Arc;

static X: AtomicI32 = AtomicI32::new(0);
static Y: AtomicI32 = AtomicI32::new(0);

fn taskset(cpuid: usize) {
    core_affinity::set_for_current(core_affinity::CoreId { id: cpuid });
}

struct ResultStore {
    result: HashMap<(i32, i32), u128>,
}

impl ResultStore {
    fn new() -> Self {
        Self {
            result: HashMap::new(),
        }
    }

    fn add(&mut self, x: i32, y: i32) {
        if let Some(c) = self.result.get_mut(&(x, y)) {
            *c += 1u128;
        } else {
            self.result.insert((x, y), 1u128);
        }
    }
    fn print(&self) {
        let sum = self.result.values().sum::<u128>();
        println!("b_iter: {}", sum);

        for ((x, y), c) in &self.result {
            println!(
                "({:2}, {:2}) = {:10}  {:.3}%",
                x,
                y,
                c,
                (c * 100) as f64 / sum as f64
            );
        }
    }
}

fn a(running: Arc<AtomicBool>, cpuid: usize) -> u128 {
    taskset(cpuid);
    let mut count = 0u128;
    while running.load(Relaxed) {
        count += 1;
        X.store(10, Relaxed);
        Y.store(20, Relaxed);
    }
    count
}

fn b(running: Arc<AtomicBool>, cpuid: usize) -> ResultStore {
    taskset(cpuid);
    let mut result_store = ResultStore::new();
    while running.load(Relaxed) {
        let y = Y.load(Relaxed);
        let x = X.load(Relaxed);
        X.store(0, Relaxed);
        Y.store(0, Relaxed);
        result_store.add(x, y);
    }

    result_store
}

fn main() {
    let running = Arc::new(AtomicBool::new(true));

    let a_th = {
        let running = running.clone();
        std::thread::spawn(move || a(running, 1))
    };

    let b_th = {
        let running = running.clone();
        std::thread::spawn(move || b(running, 2))
    };

    println!("Running for 10s...");
    std::thread::sleep(std::time::Duration::from_secs(5));

    running.store(false, Relaxed);

    let result_store = b_th.join().unwrap();
    let a_iter = a_th.join().unwrap();

    println!("a_iter: {}", a_iter);
    result_store.print();
}
