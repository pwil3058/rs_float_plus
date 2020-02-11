// Copyright 2020 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>
///! Traits to make numeric generic code easier to implement
pub mod float_plus;
pub mod unsigned_plus;

/// Constants that are applicable to numbers in general
pub trait NumberConstants {
    /// How many bytes does this type occupy
    const BYTES: usize;
    /// Approximate number of significant digits in base 10.
    const DIGITS: u32;
    /// Largest finite value for this type.
    const MAX: Self;
    /// Smallest finite value for this type.
    const MIN: Self;

    /// Other values useful in generic code
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const THREE: Self;
}
