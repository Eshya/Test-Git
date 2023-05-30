use std::time::{Instant, Duration};

fn perform_cpu_intensive_task() {
    // This function simulates a CPU-intensive task
    let mut sum = 0;
    for _ in 0..1_000_000_000 {
        sum += 1;
    }
}

fn main() {
    let start_time = Instant::now();

    // Perform the CPU-intensive task
    perform_cpu_intensive_task();

    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);
}
