use crate::Point;
use alloc::vec::Vec;

use super::MAX_COMPACT_WIDTH;

/// The odd multiples of a point that wNAF multiplication takes, kept by the caller.
#[derive(Clone, Debug)]
pub struct WNafTable<P> {
    width: usize,
    precomputed: Vec<P>,
    precomputed_negated: Option<Vec<P>>,
}

impl<P: Point> WNafTable<P> {
    /// `P, 3P, 5P, ...` for `width` from 2 to 16, and their negations when
    /// `include_negated`. Variable time: only for a public point.
    pub fn new(point: &P, width: usize, include_negated: bool) -> Self {
        assert!(
            (2..=MAX_COMPACT_WIDTH).contains(&width),
            "wNAF table width must be in the range 2..=16"
        );
        let count = 1_usize << (width - 2);
        let twice = point.twice();
        let mut precomputed = Vec::with_capacity(count);
        let mut current = point.clone();
        precomputed.push(current.clone());
        for _ in 1..count {
            current = current.add(&twice);
            precomputed.push(current.clone());
        }
        crate::algo::normalize_all(&mut precomputed)
            .expect("precomputed points share a valid curve");
        let precomputed_negated =
            include_negated.then(|| precomputed.iter().map(Point::negate).collect::<Vec<_>>());
        Self {
            width,
            precomputed,
            precomputed_negated,
        }
    }

    /// The window width. Constant time.
    pub const fn width(&self) -> usize {
        self.width
    }

    /// The multiples `P, 3P, 5P, ...`. Constant time.
    pub fn precomputed(&self) -> &[P] {
        &self.precomputed
    }

    /// Their negations, or `None` when they were not asked for. Constant time.
    pub fn precomputed_negated(&self) -> Option<&[P]> {
        self.precomputed_negated.as_deref()
    }

    pub(crate) fn select(&self, digit: i32) -> P {
        let index = digit.unsigned_abs() as usize >> 1;
        if digit < 0 {
            self.precomputed_negated.as_ref().map_or_else(
                || self.precomputed[index].negate(),
                |points| points[index].clone(),
            )
        } else {
            self.precomputed[index].clone()
        }
    }
}
