use int::Int;

pub trait LZCNT {
    fn lzcnt(self) -> Self;
}

impl<T: Int> LZCNT for T {
    fn lzcnt(self) -> T {
        self.leading_zeros()
    }
}

/// Counts the leading most significant zero bits.
///
/// When the operand is zero, it returns its size in bits.
///
/// **Keywords**: Count leading zeros, bit scan reverse, find last set.
///
/// # Intrinsic (when available: ABM / SSE4.2)
///
/// [`LZCNT`](http://www.felixcloutier.com/x86/LZCNT.html): Count the number of
/// leading zero bits (supports 16/32/64 bit registers).
///
/// # Example
/// ```
/// use bitintr::lzcnt;
///
/// assert_eq!(lzcnt(0b0101_1010u16), 9u16);
/// ```
pub fn lzcnt<T: LZCNT>(x: T) -> T {
    LZCNT::lzcnt(x)
}
