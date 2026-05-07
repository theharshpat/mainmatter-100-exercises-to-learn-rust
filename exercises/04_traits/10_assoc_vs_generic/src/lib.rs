// TODO: Define a new trait, `Power`, that has a method `power` that raises `self`
//  to the power of `n`.
//  The trait definition and its implementations should be enough to get
//  the tests to compile and pass.
//
// Recommendation: you may be tempted to write a generic implementation to handle
// all cases at once. However, this is fairly complicated and requires the use of
// additional crates (i.e. `num-traits`).
// Even then, it might be preferable to use a simple macro instead to avoid
// the complexity of a highly generic implementation. Check out the
// "Little book of Rust macros" (https://veykril.github.io/tlborm/) if you're
// interested in learning more about it.
// You don't have to though: it's perfectly okay to write three separate
// implementations manually. Venture further only if you're curious.


// My initial approach - passes tests but not optimal
//
// Mental model at this stage:
// - `Power<T>` means "a family of traits parameterized by T"
// - Therefore:
//     Power<u16>
//     Power<u32>
//     Power<&u32>
//   are effectively different trait contracts.
//
// - That's why these are all legal simultaneously:
//     impl Power<u16> for u32
//     impl Power<u32> for u32
//     impl Power<&u32> for u32
//
// - Generic parameter `T` is the INPUT type.
// - We use a generic parameter because we want MULTIPLE implementations
//   for different exponent/input types.
//
// - At this stage I used:
//     fn power(self, ...)
//   which consumes ownership of self.
//
// - That technically works because `u32: Copy`,
//   but consuming ownership is unnecessarily restrictive.
//
// - After reading more about trait design + associated types,
//   I realized borrowing is more idiomatic here because exponentiation
//   only needs read access.
//
// - Important syntax magic:
//     2_u32.power(3)
//   still works even if method takes `&self`.
//
//   Rust auto-borrows method receivers:
//     2_u32.power(3)
//   becomes approximately:
//     (&2_u32).power(3)
//
// - General API design heuristic:
//     Prefer &self unless ownership is actually required.


// Better approach after revisiting the concepts.

trait Power<T> {

    // `T` is a generic parameter because we want MANY possible input types.
    //
    // These are all different implementations:
    //     Power<u16>
    //     Power<u32>
    //     Power<&u32>
    //
    // Generic parameters are chosen from OUTSIDE the trait impl
    // based on the caller/input types.
    //
    // Changed:
    //     self -> &self
    //
    // Reason:
    // - exponentiation does not logically require ownership
    // - method only needs temporary read access
    // - borrowing is less restrictive and more reusable
    // - more idiomatic for non-consuming operations
    //
    // Mental model:
    //     self  => "I take ownership"
    //     &self => "I only need read access"
    //
    // Returning `Self` here means:
    // - output type is always the implementing type
    // - simpler than using an associated Output type
    //
    // This is okay for this exercise because:
    //     u32 ^ anything -> u32
    //
    // If we later needed:
    //     some_type.power(x) -> different_type
    // then we'd introduce:
    //     type Output;
    //
    // Similar to std::ops::Add:
    //     trait Add<Rhs> {
    //         type Output;
    //     }
    //
    // Associated types are used when the output type must be
    // UNIQUELY determined by the implementation.
    fn power(&self, n: T) -> Self;
}

impl Power<u16> for u32 {

    // `u16` exponent version.
    //
    // Compiler selects this impl because exponent type is `u16`.
    //
    // `.pow()` expects u32 exponent,
    // so convert u16 -> u32 using `.into()`.
    fn power(&self, n: u16) -> Self {
        self.pow(n.into())
    }
}

impl Power<u32> for u32 {

    // Direct u32 exponent implementation.
    fn power(&self, n: u32) -> Self {
        self.pow(n)
    }
}

impl Power<&u32> for u32 {

    // Borrowed exponent version.
    //
    // `n` is `&u32`, but `.pow()` needs owned `u32`,
    // so dereference using `*n`.
    //
    // Could also delegate:
    //
    //     self.power(*n)
    //
    // which would reuse the `Power<u32>` impl.
    //
    // That delegation style is often more idiomatic because it
    // centralizes the actual logic in one implementation.
    fn power(&self, n: &u32) -> Self {
        self.pow(*n)
    }
}

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
