use core::ops::{BitOrAssign, ShlAssign};

use embassy_rp::pac::ROSC;
use num_traits::{ConstOne, ConstZero};

macro_rules! impl_bits {
    ($($t:ident),+) => {
        $(
            impl Bits for $t {
                const BITS: u32 = $t::BITS;
            }
        )+
    };
}

pub trait Bits {
    const BITS: u32;
}

impl_bits!(u8, u16, u32, u64, i8, i16, i32, i64);

pub trait Random {
    fn random() -> Self;
}

impl<T> Random for T
where
    T: Bits + ConstZero + ConstOne + ShlAssign + BitOrAssign + From<bool>,
{
    fn random() -> Self {
        let random_reg = ROSC.randombit();
        let mut acc = Self::ZERO;
        for _ in 0..Self::BITS {
            acc <<= Self::ONE;
            acc |= random_reg.read().randombit().into();
        }
        acc
    }
}

#[inline]
pub fn random<T: Random>() -> T {
    T::random()
}

/// Returns a random `f32` value between 0.0 and 1.0.
#[inline]
pub fn random_f32() -> f32 {
    u32::random() as f32 / u32::MAX as f32
}

#[inline]
pub fn random_option<T>(map_fn: impl FnOnce() -> T) -> Option<T> {
    let random_reg = embassy_rp::pac::ROSC.randombit();

    if random_reg.read().randombit() {
        Some(map_fn())
    } else {
        None
    }
}
