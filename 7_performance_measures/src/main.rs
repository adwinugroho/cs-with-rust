use std::thread::sleep;
use std::time::{Duration, SystemTime};

fn main() {
    println!("Example of Latency and Throughput");

    let now = SystemTime::now();
    let mut i = 0;
    let max = 10;
    loop {
        delay_function();
        i += 1;
        if i == max {
            break;
        }
    }
    let duration = now.elapsed().expect("failed to check throughput time");
    println!("waktu total: {:?}", duration);
    let total_tasks = (max - 1) as f64;
    let throughput = total_tasks / duration.as_secs_f64();
    println!("throughput (tugas per detik): {:.3}", throughput);
}

fn delay_function() {
    sleep(Duration::from_millis(100)); // latency = 100 ms tiap kali dipanggil
}
