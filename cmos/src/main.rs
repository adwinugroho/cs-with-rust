fn main() {
    println!("Example of CMOS");
    println!("NOT GATE");
    println!("NOT(false) = {}", func_not(false)); // true
    println!("NOT(true) = {}", func_not(true)); // false

    println!("NAND GATE");
    println!("NAND(false, false) = {}", func_nand(false, false)); // true
    println!("NAND(false, true) = {}", func_nand(false, true)); // true
    println!("NAND(true, false) = {}", func_nand(true, false)); // true
    println!("NAND(true, true) = {}", func_nand(true, true)); // false
}

fn func_not(a: bool) -> bool {
    return !a;
}

fn func_nand(a: bool, b: bool) -> bool {
    return !(a && b);
}
