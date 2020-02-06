// Copyright 2019 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>
#[macro_export]
macro_rules! assert_approx_eq {
    ($left:expr, $right:expr) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val).approx_eq(&*right_val, None) {
                    panic!(
                        "assertion failed: `left.approx_eq(right)` \
                         (left: `{:?}`, right: `{:?}`)",
                        &*left_val, &*right_val
                    )
                }
            }
        }
    }};
    ($left:expr, $right:expr,) => {{
        $crate::assert_approx_eq!($left, $right)
    }};
    ($left:expr, $right:expr, $max_diff:expr) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val).approx_eq(&*right_val, Some($max_diff)) {
                    panic!(
                        "assertion failed: `left.approx_eq(right)` \
                         (left: `{:?}`, right: `{:?}`)",
                        &*left_val, &*right_val
                    )
                }
            }
        }
    }};
    ($left:expr, $right:expr, $max_diff:expr,) => {{
        $crate::assert_approx_eq!($left, $right, $max_diff)
    }};
}

#[macro_export]
macro_rules! assert_approx_ne {
    ($left:expr, $right:expr) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if (*left_val).approx_eq(&*right_val, None) {
                    panic!(
                        "assertion failed: `left.approx_eq(right)` \
                         (left: `{:?}`, right: `{:?}`)",
                        &*left_val, &*right_val
                    )
                }
            }
        }
    }};
    ($left:expr, $right:expr,) => {{
        $crate::assert_approx_ne!($left, $right)
    }};
    ($left:expr, $right:expr, $max_diff:expr) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if (*left_val).approx_eq(&*right_val, Some($max_diff)) {
                    panic!(
                        "assertion failed: `left.approx_eq(right)` \
                         (left: `{:?}`, right: `{:?}`)",
                        &*left_val, &*right_val
                    )
                }
            }
        }
    }};
    ($left:expr, $right:expr, $max_diff:expr,) => {{
        $crate::assert_approx_ne!($left, $right, $max_diff)
    }};
}

pub use num_traits::{cast::*, float::*, NumAssignOps, NumOps};

pub trait FloatPlus:
    Float + NumOps + NumAssignOps + NumCast + FromPrimitive + FloatApproxEq<Self>
{
    /// Approximate number of significant digits in base 10.
    const DIGITS: u32;
    /// Machine epsilon value for `FloatPlus`.
    const EPSILON: Self;
    /// Infinity (∞).
    const INFINITY: Self;
    /// Number of significant digits in base 2.
    const MANTISSA_DIGITS: u32;
    /// Largest finite `FloatPlus` value.
    const MAX: Self;
    /// Maximum possible power of 10 exponent.
    const MAX_10_EXP: i32;
    /// Maximum possible power of 2 exponent.
    const MAX_EXP: i32;
    /// Smallest finite `FloatPlus` value.
    const MIN: Self;
    /// Minimum possible normal power of 10 exponent.
    const MIN_10_EXP: i32;
    /// One greater than the minimum possible normal power of 2 exponent.
    const MIN_EXP: i32;
    /// Smallest positive normal `FloatPlus` value.
    const MIN_POSITIVE: Self;
    /// Not a Number (NaN).
    const NAN: Self;
    /// Negative infinity (-∞).
    const NEG_INFINITY: Self;
    /// The radix or base of the internal representation of `FloatPlus`.
    const RADIX: u32;

    /// Mathematical constants
    ///
    /// Euler's number (e)
    const E: Self;
    /// 1/π
    const FRAC_1_PI: Self;
    /// 2/π
    const FRAC_2_PI: Self;
    /// 2/sqrt(π)
    const FRAC_2_SQRT_PI: Self;
    /// 1/sqrt(2)
    const FRAC_1_SQRT_2: Self;
    /// π/2
    const FRAC_PI_2: Self;
    /// π/3
    const FRAC_PI_3: Self;
    /// π/4
    const FRAC_PI_4: Self;
    /// π/6
    const FRAC_PI_6: Self;
    /// π/8
    const FRAC_PI_8: Self;
    /// ln(2)
    const LN_2: Self;
    /// ln(10)
    const LN_10: Self;
    /// log2(e)
    const LOG2_E: Self;
    /// log10(e)
    const LOG10_E: Self;
    /// Archimedes' constant (π)
    const PI: Self;
    /// sqrt(2)
    const SQRT_2: Self;

    /// Other values useful in generic float code
    const ZERO: Self;
    const ONE: Self;
    const NEG_ONE: Self;
    const TWO: Self;
    const HALF: Self;
    const THREE: Self;
    const SQRT_3: Self;
    const TWO_PI: Self;
}

impl FloatPlus for f64 {
    /// Approximate number of significant digits in base 10.
    const DIGITS: u32 = std::f64::DIGITS;
    /// Machine epsilon value for `FloatPlus`.
    const EPSILON: Self = std::f64::EPSILON;
    /// Infinity (∞).
    const INFINITY: Self = std::f64::INFINITY;
    /// Number of significant digits in base 2.
    const MANTISSA_DIGITS: u32 = std::f64::MANTISSA_DIGITS;
    /// Largest finite `FloatPlus` value.
    const MAX: Self = std::f64::MAX;
    /// Maximum possible power of 10 exponent.
    const MAX_10_EXP: i32 = std::f64::MAX_10_EXP;
    /// Maximum possible power of 2 exponent.
    const MAX_EXP: i32 = std::f64::MAX_EXP;
    /// Smallest finite `FloatPlus` value.
    const MIN: Self = std::f64::MIN;
    /// Minimum possible normal power of 10 exponent.
    const MIN_10_EXP: i32 = std::f64::MIN_10_EXP;
    /// One greater than the minimum possible normal power of 2 exponent.
    const MIN_EXP: i32 = std::f64::MIN_EXP;
    /// Smallest positive normal `FloatPlus` value.
    const MIN_POSITIVE: Self = std::f64::MIN_POSITIVE;
    /// Not a Number (NaN).
    const NAN: Self = std::f64::NAN;
    /// Negative infinity (-∞).
    const NEG_INFINITY: Self = std::f64::NEG_INFINITY;
    /// The radix or base of the internal representation of `FloatPlus`.
    const RADIX: u32 = std::f64::RADIX;

    /// Mathematical constants
    ///
    /// Euler's number (e)
    const E: Self = std::f64::consts::E;
    /// 1/π
    const FRAC_1_PI: Self = std::f64::consts::FRAC_1_PI;
    /// 2/π
    const FRAC_2_PI: Self = std::f64::consts::FRAC_2_PI;
    /// 2/sqrt(π)
    const FRAC_2_SQRT_PI: Self = std::f64::consts::FRAC_2_SQRT_PI;
    /// 1/sqrt(2)
    const FRAC_1_SQRT_2: Self = std::f64::consts::FRAC_1_SQRT_2;
    /// π/2
    const FRAC_PI_2: Self = std::f64::consts::FRAC_PI_2;
    /// π/3
    const FRAC_PI_3: Self = std::f64::consts::FRAC_PI_3;
    /// π/4
    const FRAC_PI_4: Self = std::f64::consts::FRAC_PI_4;
    /// π/6
    const FRAC_PI_6: Self = std::f64::consts::FRAC_PI_6;
    /// π/8
    const FRAC_PI_8: Self = std::f64::consts::FRAC_PI_8;
    /// ln(2)
    const LN_2: Self = std::f64::consts::LN_2;
    /// ln(10)
    const LN_10: Self = std::f64::consts::LN_10;
    /// log2(e)
    const LOG2_E: Self = std::f64::consts::LOG2_E;
    /// log10(e)
    const LOG10_E: Self = std::f64::consts::LOG10_E;
    /// Archimedes' constant (π)
    const PI: Self = std::f64::consts::PI;
    /// sqrt(2)
    const SQRT_2: Self = std::f64::consts::SQRT_2;

    /// Other values useful in generic float code
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
    const NEG_ONE: Self = -1.0;
    const TWO: Self = 2.0;
    const HALF: Self = 0.5;
    const THREE: Self = 3.0;
    const SQRT_3: Self = 1.73205_08075_68878;
    const TWO_PI: Self = std::f64::consts::PI * 2.0;
}

impl FloatPlus for f32 {
    /// Approximate number of significant digits in base 10.
    const DIGITS: u32 = std::f32::DIGITS;
    /// Machine epsilon value for `FloatPlus`.
    const EPSILON: Self = std::f32::EPSILON;
    /// Infinity (∞).
    const INFINITY: Self = std::f32::INFINITY;
    /// Number of significant digits in base 2.
    const MANTISSA_DIGITS: u32 = std::f32::MANTISSA_DIGITS;
    /// Largest finite `FloatPlus` value.
    const MAX: Self = std::f32::MAX;
    /// Maximum possible power of 10 exponent.
    const MAX_10_EXP: i32 = std::f32::MAX_10_EXP;
    /// Maximum possible power of 2 exponent.
    const MAX_EXP: i32 = std::f32::MAX_EXP;
    /// Smallest finite `FloatPlus` value.
    const MIN: Self = std::f32::MIN;
    /// Minimum possible normal power of 10 exponent.
    const MIN_10_EXP: i32 = std::f32::MIN_10_EXP;
    /// One greater than the minimum possible normal power of 2 exponent.
    const MIN_EXP: i32 = std::f32::MIN_EXP;
    /// Smallest positive normal `FloatPlus` value.
    const MIN_POSITIVE: Self = std::f32::MIN_POSITIVE;
    /// Not a Number (NaN).
    const NAN: Self = std::f32::NAN;
    /// Negative infinity (-∞).
    const NEG_INFINITY: Self = std::f32::NEG_INFINITY;
    /// The radix or base of the internal representation of `FloatPlus`.
    const RADIX: u32 = std::f32::RADIX;

    /// Mathematical constants
    ///
    /// Euler's number (e)
    const E: Self = std::f32::consts::E;
    /// 1/π
    const FRAC_1_PI: Self = std::f32::consts::FRAC_1_PI;
    /// 2/π
    const FRAC_2_PI: Self = std::f32::consts::FRAC_2_PI;
    /// 2/sqrt(π)
    const FRAC_2_SQRT_PI: Self = std::f32::consts::FRAC_2_SQRT_PI;
    /// 1/sqrt(2)
    const FRAC_1_SQRT_2: Self = std::f32::consts::FRAC_1_SQRT_2;
    /// π/2
    const FRAC_PI_2: Self = std::f32::consts::FRAC_PI_2;
    /// π/3
    const FRAC_PI_3: Self = std::f32::consts::FRAC_PI_3;
    /// π/4
    const FRAC_PI_4: Self = std::f32::consts::FRAC_PI_4;
    /// π/6
    const FRAC_PI_6: Self = std::f32::consts::FRAC_PI_6;
    /// π/8
    const FRAC_PI_8: Self = std::f32::consts::FRAC_PI_8;
    /// ln(2)
    const LN_2: Self = std::f32::consts::LN_2;
    /// ln(10)
    const LN_10: Self = std::f32::consts::LN_10;
    /// log2(e)
    const LOG2_E: Self = std::f32::consts::LOG2_E;
    /// log10(e)
    const LOG10_E: Self = std::f32::consts::LOG10_E;
    /// Archimedes' constant (π)
    const PI: Self = std::f32::consts::PI;
    /// sqrt(2)
    const SQRT_2: Self = std::f32::consts::SQRT_2;

    /// Other values useful in generic float code
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
    const NEG_ONE: Self = -1.0;
    const TWO: Self = 2.0;
    const HALF: Self = 0.5;
    const THREE: Self = 3.0;
    const SQRT_3: Self = 1.732_050_8;
    const TWO_PI: Self = std::f32::consts::PI * 2.0;
}

pub trait FloatApproxEq<F: FloatPlus>: std::cmp::PartialEq {
    const DEFAULT_MAX_DIFF: F = F::EPSILON;

    fn abs_diff(&self, other: &Self) -> F;
    fn rel_diff_scale_factor(&self, other: &Self) -> F;

    fn approx_eq(&self, other: &Self, max_diff: Option<F>) -> bool {
        if self.eq(other) {
            return true;
        }
        let max_diff = if let Some(max_diff) = max_diff {
            max_diff
        } else {
            Self::DEFAULT_MAX_DIFF
        };
        let abs_diff = self.abs_diff(other);
        if abs_diff <= max_diff {
            return true;
        }
        if abs_diff <= max_diff * self.rel_diff_scale_factor(other) {
            return true;
        }
        false
    }
}

impl FloatApproxEq<f64> for f64 {
    fn abs_diff(&self, other: &Self) -> f64 {
        (self - other).abs()
    }

    fn rel_diff_scale_factor(&self, other: &Self) -> f64 {
        self.abs().max(other.abs())
    }
}

impl FloatApproxEq<f32> for f32 {
    fn abs_diff(&self, other: &Self) -> f32 {
        (self - other).abs()
    }

    fn rel_diff_scale_factor(&self, other: &Self) -> f32 {
        self.abs().max(other.abs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct WrappedFloat<F: FloatPlus>(F);

    impl<F: FloatPlus> std::cmp::PartialEq for WrappedFloat<F> {
        fn eq(&self, other: &Self) -> bool {
            self.0.eq(&other.0)
        }
    }

    impl<F: FloatPlus + std::fmt::Debug> FloatApproxEq<F> for WrappedFloat<F> {
        fn abs_diff(&self, other: &Self) -> F {
            (self.0 - other.0).abs()
        }

        fn rel_diff_scale_factor(&self, other: &Self) -> F {
            self.0.abs().max(other.0.abs())
        }
    }

    #[test]
    fn f64_works() {
        assert_approx_eq!(
            10.0_f64 * 0.1,
            0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1
        );
        assert_ne!(
            10.0_f64 * 0.1,
            0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1
        );
    }

    #[test]
    fn f32_works() {
        assert_approx_eq!(
            10.0_f32 * 0.1,
            0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1
        );
        assert_ne!(
            10.0_f32 * 0.1,
            0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1
        );
    }

    #[test]
    fn wrapped_f64_works() {
        let sum = WrappedFloat::<f64>(0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1);
        let mul = WrappedFloat::<f64>(0.1 * 10.0);
        assert!(sum.approx_eq(&mul, None));
        assert_approx_eq!(sum, mul);
        assert_ne!(sum.0, mul.0);
    }

    #[test]
    fn wrapped_f32_works() {
        let sum = WrappedFloat::<f32>(0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1 + 0.1);
        let mul = WrappedFloat::<f32>(0.1 * 10.0);
        assert!(sum.approx_eq(&mul, None));
        assert_approx_eq!(sum, mul);
        assert_ne!(sum.0, mul.0);
    }
}
