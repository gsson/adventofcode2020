#![feature(const_generics)]

use std::fmt::{Debug, Formatter};
use std::ops::{RangeBounds, AddAssign, BitOrAssign, BitAndAssign, SubAssign};
use std::ops::Bound::{Included, Excluded, Unbounded};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BitIndex<const N: usize>([u128; N]);

fn from_bound<T: RangeBounds<usize>>(range: &T) -> usize {
    match range.start_bound() {
        Included(i) => *i,
        Excluded(i) => i + 1,
        Unbounded => 0,
    }
}

fn to_bound<const N: usize, T: RangeBounds<usize>>(range: &T) -> usize {
    match range.end_bound() {
        Included(i) => *i,
        Excluded(i) => i - 1,
        Unbounded => N * 128 - 1
    }
}

#[inline]
fn pos(v: usize) -> (usize, u128) {
    let i = v >> 7;
    let bit = 1u128 << (v & 0x7f);
    (i, bit)
}

#[inline]
fn assign_op<const N: usize, F>(lhs: &mut [u128; N], rhs: &[u128; N], op: F)
    where
        F: Fn(u128, u128) -> u128 {
    lhs.iter_mut()
        .zip(rhs.iter())
        .for_each(move |(l, r)| *l = op(*l, *r));
}


impl<const N: usize> BitIndex<N> {
    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|v| *v == 0u128)
    }

    pub fn empty() -> Self {
        Self([0u128; N])
    }

    pub fn from_ranges<T: RangeBounds<usize>, I: IntoIterator<Item=T>>(ranges: I) -> Self {
        let mut a = [0u128; N];
        for range in ranges {
            let (from_i, from_bit) = pos(from_bound(&range));
            let (to_i, to_bit) = pos(to_bound::<N, T>(&range));
            let from_mask = !(from_bit - 1);
            let to_mask = (to_bit - 1) | to_bit;
            if from_i == to_i {
                a[from_i] |= from_mask & to_mask
            } else {
                a[from_i] |= from_mask;
                a[to_i] |= to_mask;
                for i in from_i + 1..to_i {
                    a[i] |= !0u128;
                }
            }
        }
        Self(a)
    }

    pub fn from_range<R: RangeBounds<usize>>(range: R) -> Self {
        let mut a = [0u128; N];
        let (from_i, from_bit) = pos(from_bound(&range));
        let (to_i, to_bit) = pos(to_bound::<N, _>(&range));
        let from_mask = !(from_bit - 1);
        let to_mask = (to_bit - 1) | to_bit;
        if from_i == to_i {
            a[from_i] |= from_mask & to_mask
        } else {
            a[from_i] |= from_mask;
            a[to_i] |= to_mask;
            for i in from_i + 1..to_i {
                a[i] |= !0u128;
            }
        }

        Self(a)
    }

    #[inline]
    pub fn contains(&self, v: usize) -> bool {
        let (i, bit) = pos(v);
        self.0[i] & bit != 0
    }

    pub fn len(&self) -> usize {
        self.0.iter()
            .map(|v| v.count_ones() as usize)
            .sum()
    }

    pub fn iter(&self) -> impl Iterator<Item=usize> + '_ {
        fn b(offset: usize, mut bits: u128) -> impl Iterator<Item=usize> {
            std::iter::from_fn(move || {
                if bits != 0 {
                    let lsb = bits.trailing_zeros();
                    bits ^= 1 << lsb;
                    Some(offset + lsb as usize)
                } else {
                    None
                }
            })
        }
        self.0.iter()
            .enumerate()
            .filter(|(_, s)| **s != 0)
            .flat_map(move |(i, s)| b(i << 7, *s))
    }
}

impl<const N: usize> AddAssign<usize> for BitIndex<N> {
    #[inline]
    fn add_assign(&mut self, rhs: usize) {
        let (i, bit) = pos(rhs);
        self.0[i] |= bit;
    }
}

impl<const N: usize> BitOrAssign<BitIndex<N>> for BitIndex<N> {
    #[inline]
    fn bitor_assign(&mut self, rhs: BitIndex<N>) {
        assign_op(&mut self.0, &rhs.0, |l, r| l | r );
    }
}

impl<const N: usize> BitOrAssign<&BitIndex<N>> for BitIndex<N> {
    #[inline]
    fn bitor_assign(&mut self, rhs: &BitIndex<N>) {
        assign_op(&mut self.0, &rhs.0, |l, r| l | r );
    }
}

impl<const N: usize> BitAndAssign<BitIndex<N>> for BitIndex<N> {
    #[inline]
    fn bitand_assign(&mut self, rhs: BitIndex<N>) {
        assign_op(&mut self.0, &rhs.0, |l, r| l & r );
    }
}

impl<const N: usize> BitAndAssign<&BitIndex<N>> for BitIndex<N> {
    #[inline]
    fn bitand_assign(&mut self, rhs: &BitIndex<N>) {
        assign_op(&mut self.0, &rhs.0, |l, r| l & r );
    }
}


impl<const N: usize> SubAssign<BitIndex<N>> for BitIndex<N> {
    #[inline]
    fn sub_assign(&mut self, rhs: BitIndex<N>) {
        assign_op(&mut self.0, &rhs.0, |l, r| l & !r);
    }
}

impl<const N: usize> SubAssign<&BitIndex<N>> for BitIndex<N> {
    #[inline]
    fn sub_assign(&mut self, rhs: &BitIndex<N>) {
        assign_op(&mut self.0, &rhs.0, |l, r| l & !r);
    }
}

impl<const N: usize> SubAssign<usize> for BitIndex<N> {
    #[inline]
    fn sub_assign(&mut self, rhs: usize) {
        let (i, bit) = pos(rhs);
        self.0[i] &= !bit;
    }
}


impl<const N: usize> Debug for BitIndex<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        for (i, n) in self.0.iter().enumerate() {
            if *n != 0 {
                write!(f, " {:3} {:b}", i * 128, n)?
            }
        }
        f.write_str(" ]")
    }
}
