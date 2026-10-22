//! BC's `FixedPointCombMultiplier` and `FixedPointUtilities`: the scalar, as wide as the
//! order, is laid out as a comb of w rows and d columns, and each column's w bits index a
//! table of 2^w points. Each column then takes one doubling and one addition, d of each in
//! all, instead of a doubling per bit. Building the table costs about one scalar
//! multiplication, so the comb pays when a base is reused.

use super::BaseMultiplier;
use crate::algo::normalize_all;
use crate::{Curve, Point, Scalar};
use alloc::vec;
use alloc::vec::Vec;
use tc_bigint::PaddedBigUint;
use tc_constant_time::{ConditionallySelectable, ConstantTimeEq};

/// BC's `FixedPointCombMultiplier` with a lookup that reads the whole table.
///
/// The points must select in constant time, as those of the Fp Ct curves do. That bound
/// covers only the lookup: whether the group operations are constant time is up to the
/// point type, and the Fp Ct curves use complete formulas. A scalar wider than the order
/// panics, as BC throws; that check reveals only whether the scalar is too wide.
pub struct CombCt<C: Curve> {
    table: Table<C::Point>,
}

/// Constant time in the scalar. Building the table is variable time, only for a public
/// base.
impl<C> BaseMultiplier<C> for CombCt<C>
where
    C: Curve<Scalar = PaddedBigUint>,
    C::Point: ConditionallySelectable,
{
    fn new(base: C::Point) -> Self {
        Self {
            table: Table::new(base),
        }
    }

    fn multiply(&self, scalar: &PaddedBigUint) -> C::Point {
        let table = &self.table;
        let width = scalar.as_limbs().len() * tc_bigint::Word::BITS as usize;
        let mut high = 0u8;
        for j in table.size..width {
            high |= u8::from(scalar.bit(j as u32));
        }
        assert!(high == 0, "the comb takes scalars no wider than the order");
        table.multiply(
            |j| u32::from(scalar.bit(j as u32)),
            |points, index| {
                let mut selected = points[0].clone();
                for (i, entry) in (0u32..).zip(points).skip(1) {
                    selected = C::Point::conditional_select(&selected, entry, i.ct_eq(&index));
                }
                selected
            },
        )
    }
}

/// BC's `FixedPointCombMultiplier` with a lookup by index. A scalar wider than the order
/// panics, as BC throws.
pub struct CombVar<C: Curve> {
    table: Table<C::Point>,
}

/// Variable time: only for public values, the base and the scalar.
impl<C: Curve> BaseMultiplier<C> for CombVar<C> {
    fn new(base: C::Point) -> Self {
        Self {
            table: Table::new(base),
        }
    }

    fn multiply(&self, scalar: &C::Scalar) -> C::Point {
        let table = &self.table;
        assert!(
            scalar.bit_length() <= table.size,
            "the comb takes scalars no wider than the order"
        );
        table.multiply(
            |j| u32::from(scalar.test_bit(j)),
            |points, index| points[index as usize].clone(),
        )
    }
}

// BC's `FixedPointPreCompInfo`: points[i] = P + Σ 2^(bd)·P over the bits b of i, and
// offset = P − 2^d·P, which cancels the P every column adds. No entry is infinity.
struct Table<P> {
    points: Vec<P>,
    offset: P,
    size: usize,
    width: usize,
    columns: usize,
}

impl<P: Point> Table<P> {
    fn new(base: P) -> Self {
        let size = base
            .curve()
            .order()
            .expect("the comb needs the order")
            .bit_length();
        let width = if size > 250 { 6 } else { 5 };
        let columns = size.div_ceil(width);

        let mut pow2 = Vec::with_capacity(width + 1);
        pow2.push(base.clone());
        for i in 1..width {
            let next = pow2[i - 1].times_pow2(columns);
            pow2.push(next);
        }
        pow2.push(pow2[0].add(&pow2[1].negate()));
        let _ = normalize_all(&mut pow2);

        let n = 1 << width;
        let mut points = vec![base; n];
        for bit in (0..width).rev() {
            let step = 1 << bit;
            for i in (step..n).step_by(step << 1) {
                points[i] = points[i - step].add(&pow2[bit]);
            }
        }
        let _ = normalize_all(&mut points);

        Self {
            points,
            offset: pow2.pop().expect("the offset was pushed last"),
            size,
            width,
            columns,
        }
    }

    // Column i, from 1 to d, indexes by the bits j = (w − 1 − t)·d + (d − i) for t from 0
    // to w − 1, the highest first; the loop counts follow the order's width alone.
    fn multiply(&self, bit: impl Fn(usize) -> u32, lookup: impl Fn(&[P], u32) -> P) -> P {
        let full = self.width * self.columns;
        let mut result = self.offset.infinity();
        for i in 1..=self.columns {
            let mut index = 0;
            for t in 0..self.width {
                index = (index << 1) | bit(full - i - t * self.columns);
            }
            result = result.twice_plus(&lookup(&self.points, index));
        }
        result.add(&self.offset)
    }
}
