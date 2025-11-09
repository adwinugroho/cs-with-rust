fn main() {
    println!("Cek bit!");
    println!("koin adil: {:?}", info_bits(0.5));
    println!("dadu 6 sisi: {:?}", info_bits(1.0 / 6.0));
    println!("kejadian pasti: {:?}", info_bits(1.0));
}

fn info_bits(probability: f64) -> f64 {
    if probability == 0.0 || probability > 1.0 {
        return 0.0;
    }

    return f64::log2(1.0 / probability);
}
