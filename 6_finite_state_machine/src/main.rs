#[derive(Debug, PartialEq)]
enum State {
    ON,
    OFF,
}

struct ToogleSwitch {
    state: State,
}

fn main() {
    println!("Example of FSM");

    let mut swtitcher = ToogleSwitch { state: State::OFF };
    println!("Awal lampu: {:?}", swtitcher.state);

    for d in 0..5 {
        swtitcher.press();
        println!("Tekan:{}, lampu {:?}", d, swtitcher.state);
    }

    println!("Cek lampu is ON:{}", swtitcher.is_on());
}

impl ToogleSwitch {
    fn press(&mut self) {
        self.state = match self.state {
            State::ON => State::OFF,
            State::OFF => State::ON,
        };
    }

    fn is_on(&self) -> bool {
        return self.state == State::ON;
    }
}
