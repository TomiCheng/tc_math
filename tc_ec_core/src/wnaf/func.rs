use crate::{Curve, Point, Scalar};
use alloc::vec::Vec;

use super::{DEFAULT_WINDOW_SIZE_CUTOFFS, MAX_COMPACT_WIDTH, MAX_DENSE_WIDTH, WNafTable};

/// Joint sparse form of two unsigned scalars, least significant pair first.
/// Digits are in `-1..=1`. Carry bits avoid overflowing fixed-width scalars.
/// Variable time; suitable only for public scalars.
///
/// Solinas' algorithm: a digit flips sign when its residue mod 8 is 3 or 5 and
/// the other residue is 2 mod 4. BC tests `n + u == 4` there, which never holds,
/// so its output is a pair of plain NAFs rather than the joint sparse form.
pub fn generate_jsf<C: Curve>(a: &C::Scalar, b: &C::Scalar) -> Vec<[i8; 2]> {
    let mut values = [a.clone(), b.clone()];
    let mut carry = [0_i32; 2];
    let mut result = Vec::new();
    while carry != [0, 0] || values.iter().any(|v| !v.is_zero()) {
        let n = [
            (values[0].low_bits(3) as i32 + carry[0]) & 7,
            (values[1].low_bits(3) as i32 + carry[1]) & 7,
        ];
        let mut digit = [0_i8; 2];
        for i in 0..2 {
            let mut u = n[i] & 1;
            if u != 0 {
                u -= n[i] & 2;
                if matches!(n[i], 3 | 5) && n[1 - i] & 3 == 2 {
                    u = -u;
                }
            }
            if carry[i] * 2 == 1 + u {
                carry[i] ^= 1;
            }
            digit[i] = u as i8;
            values[i] = values[i].shr(1);
        }
        result.push(digit);
    }
    result
}

/// The NAF of `scalar`, of digits -1, 0 and 1, the lowest at index 0. Variable time: only
/// for public scalars.
pub fn generate_naf<C: Curve>(scalar: &C::Scalar) -> Vec<i8> {
    generate_window_naf::<C>(2, scalar)
}

/// The wNAF of `scalar` for `width` from 2 to 8, the lowest digit at index 0. Every nonzero
/// digit is odd and below `2^(width - 1)` in magnitude. Variable time: only for public
/// scalars.
pub fn generate_window_naf<C: Curve>(width: usize, scalar: &C::Scalar) -> Vec<i8> {
    assert!(
        (2..=MAX_DENSE_WIDTH).contains(&width),
        "wNAF width must be in the range 2..=8"
    );

    if scalar.is_zero() {
        return Vec::new();
    }

    // As BC, a carry stands for what a negative digit adds, rather than forming
    // `k + |digit|`, which for a full-width scalar would need one bit more.
    let mut value = scalar.clone();
    let mut digits = Vec::with_capacity(scalar.bit_length().saturating_add(1));
    let modulus = 1_i32 << width;
    let sign = modulus >> 1;
    let mut carry = false;
    let mut length = 0_usize;
    let mut position = 0_usize;

    while position <= value.bit_length() {
        if value.test_bit(position) == carry {
            position += 1;
            continue;
        }

        value = value.shr(position);
        let mut digit = value.low_bits(width) as i32;
        if carry {
            digit += 1;
        }
        carry = digit & sign != 0;
        if carry {
            digit -= modulus;
        }

        length += if length == 0 { position } else { position - 1 };
        digits.resize(length, 0);
        digits.push(digit as i8);
        length += 1;
        position = width;
    }
    digits
}

/// The compact wNAF that BC's `WNafL2RMultiplier` takes: each `i32` holds a signed digit in
/// its high 16 bits and, in its low 16, the zeros that follow the previous nonzero digit.
/// `width` runs from 2 to 16 and the scalar has fewer than `2^16` bits. Variable time: only
/// for public scalars.
pub fn generate_compact_window_naf<C: Curve>(width: usize, scalar: &C::Scalar) -> Vec<i32> {
    assert!(
        (2..=MAX_COMPACT_WIDTH).contains(&width),
        "compact wNAF width must be in the range 2..=16"
    );
    let bits = scalar.bit_length();
    assert!(bits < (1 << 16), "scalar bit length must be below 2^16");

    if scalar.is_zero() {
        return Vec::new();
    }

    let mut value = scalar.clone();
    let mut compact = Vec::with_capacity(bits / width + 1);
    let modulus = 1_i32 << width;
    let sign = modulus >> 1;
    let mut carry = false;
    let mut position = 0_usize;

    while position <= value.bit_length() {
        if value.test_bit(position) == carry {
            position += 1;
            continue;
        }

        value = value.shr(position);
        let mut digit = value.low_bits(width) as i32;
        if carry {
            digit += 1;
        }
        carry = digit & sign != 0;
        if carry {
            digit -= modulus;
        }

        let zeroes = if compact.is_empty() {
            position
        } else {
            position - 1
        };
        compact.push((digit << 16) | zeroes as i32);
        position = width;
    }
    compact
}

/// The default window width for a scalar of `bits` bits, by BC's cutoffs. Variable time:
/// only for public values, as a scalar's width is.
pub fn get_window_size(bits: usize) -> usize {
    let mut width = 0;
    while width < DEFAULT_WINDOW_SIZE_CUTOFFS.len() && bits >= DEFAULT_WINDOW_SIZE_CUTOFFS[width] {
        width += 1;
    }
    (width + 2).clamp(2, MAX_COMPACT_WIDTH)
}

/// The nonzero digits in the NAF of `scalar`. Variable time: only for public scalars.
pub fn get_naf_weight<C: Curve>(scalar: &C::Scalar) -> usize {
    generate_naf::<C>(scalar)
        .into_iter()
        .filter(|digit| *digit != 0)
        .count()
}

/// BC's left-to-right wNAF multiplication on a table built beforehand. Variable time: only
/// for public scalars.
pub fn wnaf_mul<C: Curve>(table: &WNafTable<C::Point>, scalar: &C::Scalar) -> C::Point {
    let compact = generate_compact_window_naf::<C>(table.width(), scalar);
    let mut result = table.precomputed()[0].infinity();
    let mut index = compact.len();

    if index > 1 {
        index -= 1;
        let (digit, mut zeroes) = unpack(compact[index]);
        let absolute = digit.unsigned_abs() as usize;
        if (absolute << 2) < (1_usize << table.width()) {
            let highest = usize::BITS as usize - absolute.leading_zeros() as usize;
            let scale = table.width() - highest;
            let low_bits = absolute ^ (1_usize << (highest - 1));
            let first = (1_usize << (table.width() - 1)) - 1;
            let second = (low_bits << scale) + 1;
            let sign = digit.signum();
            result = table
                .select(sign * first as i32)
                .add(&table.select(sign * second as i32));
            zeroes -= scale;
        } else {
            result = table.select(digit);
        }
        result = result.times_pow2(zeroes);
    }

    while index > 0 {
        index -= 1;
        let (digit, zeroes) = unpack(compact[index]);
        result = result.twice_plus(&table.select(digit));
        result = result.times_pow2(zeroes);
    }
    result
}

/// [`wnaf_mul`] on a table built for this call alone, its width set by the scalar. To
/// multiply one point many times, keep a [`WNafTable`] instead. Variable time: only for
/// public scalars.
pub fn wnaf_mul_point<C: Curve>(point: &C::Point, scalar: &C::Scalar) -> C::Point {
    let width = get_window_size(scalar.bit_length());
    let table = WNafTable::new(point, width, true);
    wnaf_mul::<C>(&table, scalar)
}

fn unpack(compact: i32) -> (i32, usize) {
    (compact >> 16, (compact & 0xFFFF) as usize)
}
