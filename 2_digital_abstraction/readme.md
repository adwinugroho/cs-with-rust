### Digital Abstraction

- How the real world becomes 0s and 1s.
- Di dunia nyata, tidak semua sinyal sempurna 0V atau 5V. Bisa jadi ada *noise*, panas, interferensi. Nah tapi komputer tetap bisa bekerja dengan baik. Kenapa?
Karena digital abstraction, kita sepakat bahwa:
- Jika tegangan di atas ambang tertentu → kita anggap 1
- Jika tegangan di bawah ambang tertentu → kita anggap 0

Tegangan (V)
   5V  ──────────────────────
       │      HIGH (1)
   3.5V ────────────────────── ← Threshold HIGH (Vih)
       │
   1.5V ────────────────────── ← Threshold LOW (Vol)
       │      LOW (0)
   0V  ──────────────────────

- VIH (Voltage Input High): Minimal tegangan untuk dianggap HIGH.
- VOL (Voltage Output Low): Maksimal tegangan untuk dianggap LOW.
- Noise margin: Jarak antara sinyal valid dan threshold → membuat sistem tahan noise.
