# tc_modular benchmarks

## Results

Measured on 2026-10-09 on the local Windows x86-64 host, using Rust 1.98.0 and
Cargo's optimized bench profile, so a limb is 64 bits. The CPU model was not
recorded. These results describe this host and run, not a cross-platform
ranking.

Each of the 40 cases used Criterion with a 3-second warm-up, a 15-second
measurement window and 100 samples; Criterion estimated that the 2048-bit
`FixedBigUint` even-modulus inverse needed about 24 seconds for its samples.
Values below are Criterion's central time estimates. The odd modulus fills
every limb and has its top bit set, the even one is the same with its low bit
cleared, and the operands and exponent fill every limb but the top bit, so
they lie below both moduli.

**Time per operation at 256 bits; lower is better.**

| Operation | `FixedBigUint` | `PaddedBigUint` | `BigUint` |
| --- | ---: | ---: | ---: |
| `mod_mul` | 22.5 µs | 114.4 µs | 1.02 µs |
| `mod_pow` | 61.5 µs | 102.7 µs | 61.9 µs |
| `mod_inverse`, odd modulus | 8.08 µs | 6.61 µs | 120.5 µs |
| `mod_inverse`, even modulus | 59.3 µs | 49.0 µs | 158.2 µs |
| Montgomery parameters `new` | 6.10 µs | 6.60 µs | — |
| Montgomery form `mul` | 63.6 ns | 254.2 ns | 306.2 ns |
| Montgomery form `pow` | 58.1 µs | 99.4 µs | 91.3 µs (`pow_vartime`) |

**Time per operation at 2048 bits; lower is better.**

| Operation | `FixedBigUint` | `PaddedBigUint` | `BigUint` |
| --- | ---: | ---: | ---: |
| `mod_mul` | 1.64 ms | 2.53 ms | 5.08 µs |
| `mod_pow` | 17.7 ms | 11.2 ms | 7.17 ms |
| `mod_inverse`, odd modulus | 348.2 µs | 284.6 µs | 1.09 ms |
| `mod_inverse`, even modulus | 2.67 ms | 2.19 ms | 1.17 ms |
| Montgomery parameters `new` | 401.4 µs | 382.2 µs | — |
| Montgomery form `mul` | 4.08 µs | 3.31 µs | 2.58 µs |
| Montgomery form `pow` | 17.0 ms | 10.6 ms | 7.39 ms (`pow_vartime`) |

The three kinds do not share one contract. Over `FixedBigUint` and
`PaddedBigUint` every operation is constant time apart from the parity of the
modulus, so their running time follows the width and not the values: `pow`
squares and multiplies at every bit of the exponent's width, and the inverses
run a fixed number of rounds. Over `BigUint`, going in and out of a form and
the inverses are variable time, for public values only, and `pow_vartime`
multiplies only where an exponent bit is set. `BigUint` is here for scale, not
as an alternative for secrets.

`mod_mul` over the constant-time integers never forms the double-width
product: it doubles and adds once per bit of the right operand, reducing at
every step, which is why it costs far more than a Montgomery `mul`. Repeated
products modulo one modulus belong in a Montgomery form. `mod_pow` with an odd
modulus works out the parameters on each call, so its cost is close to the
form's `pow` plus `new`. An even modulus takes the slower binary inverse
rather than safegcd.

At 2048 bits the padded forms ran faster than the fixed ones in this run;
`FixedMontyForm` holds its own copy of the parameters on the stack while
`PaddedMontyForm` borrows them, and the difference may come from that layout.
These are single-operation measurements, not constant-time verification. Some
samples were outliers; system load and CPU behaviour can affect small
differences.

## Running the benchmarks

Run every case:

```powershell
cargo bench -p tc_modular --bench modular --features alloc --locked
```

Run one width or one type, for example the 256-bit Montgomery forms:

```powershell
cargo bench -p tc_modular --bench modular --features alloc --locked -- '^256 bits/.*MontyForm'
```

The full suite takes far longer than its 40 measurement windows suggest, as
Criterion stretches the slow 2048-bit cases, such as `mod_pow`, `pow` and the
even-modulus inverses, to complete their samples. For a smoke test without
performance measurement:

```powershell
cargo bench -p tc_modular --bench modular --features alloc --locked -- --test
```
