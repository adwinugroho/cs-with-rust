const VIH: f64 = 2.5; // Voltage Input High
const VOL: f64 = 0.5; // Voltage Output Low

fn main() {
    println!("Digital Abstraction dengan noise margin~");
    let arr_voltages: [f64; 5] = [0.1, 0.4, 2.4, 2.6, 1.8]; // random voltage
    for d in arr_voltages {
        match digitalize(d) {
            Ok(state) => println!("{:.1}V → {}", d, if state { "HIGH (1)" } else { "LOW (0)" }),
            Err(e) => println!("{:.1}V → ⚠️  {}", d, e),
        }
    }
}

fn digitalize(voltage: f64) -> Result<bool, String> {
    if voltage >= VIH {
        return Ok(true);
    } else if voltage <= VOL {
        return Ok(false);
    } else {
        return Err(format!(
            "Voltage {:.1}V berada di noise zone ({}V - {}V)",
            voltage, VOL, VIH
        ));
    }
}
