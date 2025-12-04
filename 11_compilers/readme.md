### Compilers

- Compiler adalah program yang menerjemahkan kode tingkat tingkat tinggi (Go, Rust, C++ etc.)
menjadi kode mesin (assembly) yang bisa dijalankan oleh CPU
- Alur kerja:
Code -> Lexical analysis (tokens kata-kata kecil if, +, "hello", etc.)
Parsing -> Abstract syntax tree (AST)
Semantic analysis -> cek tipe, variabel, scope
Optimization -> sederhanakan, percepat, hapus yang redundan
Code generation -> assembly code
Binary executable -> binary
- Contoh fungsi ini:
fn add(x: f64, y: f64) -> f64 {
    return x + y;
}

1. Lexical analysis -> tokens
fn, add, (, x, ",", y dll.
2. Parsing
Function: add
  params: x (float), y (float)
  body: return (BinaryOp: x + y)
3. Code generation
add:
    movq    %rdi, %rax   ; x → RAX
    addq    %rsi, %rax   ; x + y → RAX
    ret                  ; kembalikan RAX


- Refleksi, bedanya sama interpreter apa?
Compiler vs Interpreter
Terjemahkan sekali → hasil binary | Terjemahkan setiap kali jalan
Cepat saat eksekusi | Lambat saat eksekusi
Contoh: Go, Rust, C++  | Contoh: Python, JavaScript (awalnya)

-  Cara melihat apa yang compilr lakuin:
rustc --emit=llvm-ir main.rs
