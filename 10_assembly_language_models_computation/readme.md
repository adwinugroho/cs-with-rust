### Assembly Language & Models of Computation

- Setelah belajar bagaimana RISC yang begini:
ADD  R1, R2, R3    ; R1 = R2 + R3
LOAD R4, 8(R5)     ; R4 = memory[R5 + 8]
JUMP loop          ; lompat ke label "loop"
maka sebenarnya itu adalah bahasa assembly
- Assembly adalah bahasa manusia yang langsung mewakili instruksi mesin
- Setiap baris assembly adalah satu instruksi CPU
- Hubungannya dengan ISA, Rust → dikompilasi → Assembly → dikonversi → Instruksi Mesin (Opcode) → dieksekusi oleh CPU (ISA)
- Cara agar di-compile menjadi assembly di rust dengan perintah begini:
rust --emit=asm -C opt-level=0 main.rs
```
