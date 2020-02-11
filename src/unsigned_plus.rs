// Copyright 2020 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

/// Constants that are applicable to numbers in general
impl crate::NumberConstants for u8 {
    /// How many bytes does this type occupy
    const BYTES: usize = 1;
    /// Approximate number of significant digits in base 10.
    const DIGITS: u32 = 3;
    /// Largest finite value for this type.
    const MAX: Self = std::u8::MAX;
    /// Smallest finite value for this type.
    const MIN: Self = std::u8::MIN;

    /// Other values useful in generic code
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
    const THREE: Self = 3;
}

/// Constants that are applicable to numbers in general
impl crate::NumberConstants for u16 {
    /// How many bytes does this type occupy
    const BYTES: usize = 2;
    /// Approximate number of significant digits in base 10.
    const DIGITS: u32 = 5;
    /// Largest finite value for this type.
    const MAX: Self = std::u16::MAX;
    /// Smallest finite value for this type.
    const MIN: Self = std::u16::MIN;

    /// Other values useful in generic code
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
    const THREE: Self = 3;
}

/// Constants that are applicable to numbers in general
impl crate::NumberConstants for u32 {
    /// How many bytes does this type occupy
    const BYTES: usize = 4;
    /// Approximate number of significant digits in base 10.
    const DIGITS: u32 = 10;
    /// Largest finite value for this type.
    const MAX: Self = std::u32::MAX;
    /// Smallest finite value for this type.
    const MIN: Self = std::u32::MIN;

    /// Other values useful in generic code
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
    const THREE: Self = 3;
}

/// Constants that are applicable to numbers in general
impl crate::NumberConstants for u64 {
    /// How many bytes does this type occupy
    const BYTES: usize = 8;
    /// Approximate number of significant digits in base 10.
    const DIGITS: u32 = 20;
    /// Largest finite value for this type.
    const MAX: Self = std::u64::MAX;
    /// Smallest finite value for this type.
    const MIN: Self = std::u64::MIN;

    /// Other values useful in generic code
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
    const THREE: Self = 3;
}
