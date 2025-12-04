fn main() {
    println!("Cek bit! Jumlah informasi yang didapatkan dari:");
    println!("koin adil (2 sisi): {:?}", info_bits(0.5)); // 1 bit = jumlah informasi untuk memilih antara dua kemungkinan yang sama besar.
    println!("dadu 6 sisi: {:?}", info_bits(1.0 / 6.0)); // Artinya butuh 2,58 sekian untuk menyimpan hasil dari dadu 6 sisi
    println!("kejadian pasti: {:?}", info_bits(1.0));
}

// Jumlah informasi (bit) = log₂(1/p)
// di mana p = probabilitas kejadian.
fn info_bits(probability: f64) -> f64 {
    if probability == 0.0 || probability > 1.0 {
        return 0.0;
    }

    return f64::log2(1.0 / probability);
}
