fn main() {
    println!("Sequential Logic");

    let mut ff = DFlipFlop { d: false };
    println!("Awal d={}", ff.read());

    ff.update(true, true);
    println!("Set 1: d={}", ff.read());

    ff.update(false, false);
    println!("Idle: d={}", ff.read());

    ff.update(true, false);
    println!("Set 0: d={}", ff.read());
}

struct DFlipFlop {
    d: bool,
}

impl DFlipFlop {
    fn update(&mut self, clock: bool, data: bool) {
        if clock {
            return self.d = data;
        }
    }

    fn read(&self) -> bool {
        return self.d;
    }
}
