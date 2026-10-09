# tc_bigint benchmarks

## Results

Measured on 2026-10-09 on the local Windows x86-64 host, using Rust 1.98.0 and
Cargo's optimized bench profile, so a limb is 64 bits. The CPU model was not
recorded. These results describe this host and run, not a cross-platform
ranking.

Each of the 30 cases used Criterion with a 3-second warm-up, a 15-second
measurement window and 100 samples. Values below are Criterion's central time
estimates. The left operand of `add`, `div_rem` and `rem` fills every limb but
the top bit; the factors of `mul` fill half the limbs, so that every result
fits. The `u32` on the right is `0x9e3779b9`.

**Time per operation; lower is better.**

| Operation | `FixedBigUint` 256 | `PaddedBigUint` 256 | `BigUint` 256 | `FixedBigUint` 2048 | `PaddedBigUint` 2048 | `BigUint` 2048 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `add` | 8.4 ns | 64.4 ns | 71.9 ns | 38.1 ns | 88.0 ns | 99.2 ns |
| `mul` | 27.6 ns | 81.8 ns | 64.4 ns | 1.42 µs | 1.45 µs | 368.9 ns |
| `div_rem` | 3.26 µs | 3.33 µs | 274.9 ns | 198.2 µs | 199.2 µs | 1.02 µs |
| `mul` by `u32` | 2.0 ns | 48.6 ns | 129.8 ns | 28.0 ns | 69.2 ns | 190.7 ns |
| `rem` by `u32` | 526.8 ns | 641.6 ns | 85.4 ns | 6.35 µs | 6.48 µs | 383.9 ns |

The three kinds do not share one contract. `FixedBigUint` and `PaddedBigUint`
run in constant time, so their running time follows the width and not the
value: division takes one round per bit of the dividend, and multiplication
covers every limb of both factors, zero or not. `BigUint` is variable time, for
public values only. It works on its trimmed limbs and divides by Knuth's
Algorithm D, a word at a time, which is why it wins wherever the constant-time
types pay for their fixed schedule. Its factors here are trimmed to half the
limbs, 16 at 2048 bits, below the 32 from which it switches to Karatsuba. It is
here for scale, not as an alternative for secrets.

`PaddedBigUint` tracks `FixedBigUint` within a few percent where the work is
large. On the short operations, its allocation of the result, since the
operands are borrowed, dominates.

These are single-operation measurements of the operators, not of modular or
Montgomery arithmetic, and not constant-time verification. Some samples were
outliers; system load and CPU behaviour can affect small differences.

## Running the benchmarks

Run every case:

```powershell
cargo bench -p tc_bigint --bench arithmetic --features alloc --locked
```

Run one width or one type, for example the 2048-bit `PaddedBigUint` cases:

```powershell
cargo bench -p tc_bigint --bench arithmetic --features alloc --locked -- '^2048 bits/PaddedBigUint'
```

The full suite takes roughly 9 minutes plus compilation and analysis time. For
a smoke test without performance measurement:

```powershell
cargo bench -p tc_bigint --bench arithmetic --features alloc --locked -- --test
```
