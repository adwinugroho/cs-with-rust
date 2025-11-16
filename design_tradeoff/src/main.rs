use std::collections::HashMap;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

fn main() {
    println!("Trade off");

    // let now = SystemTime::now();
    // let result_heavy = heavy_function(3);
    // let elapsed = now.elapsed().expect("failed to get duration");
    // println!("Waktu tempuh dengan heavy func:{}ms", elapsed.as_millis());
    // println!("Result heavy:{}", result_heavy);
    //
    let now_fast = SystemTime::now();
    let new_map = HashMap::new();
    let mut cache = Cache { data: new_map };
    let result_fast = cache.get(3);
    let elapsed_fast = now_fast.elapsed().expect("failed to get duration");
    println!(
        "Waktu tempuh dengan cache func:{}ms",
        elapsed_fast.as_millis()
    );
    println!("Result fast:{}", result_fast);
}

fn heavy_function(n: i32) -> i32 {
    sleep(Duration::from_millis(800)); // buat seolah-olah kerja berat
    return n * 2;
}

struct Cache {
    data: HashMap<i32, i32>,
}

impl Cache {
    fn get(&mut self, key: i32) -> i32 {
        if let Some(v) = self.data.get(&key) {
            return *v;
        }

        let value = heavy_function(key);
        self.data.insert(key, value);
        value
    }
}
