//! An extension trait for [`Option<T>`] which provides conversion to [`Either`].
//!
//! Deriving the naming conventions from [`Option::ok_or`] and [`Option::ok_or_else`] would lead to
//! symmetrical methods: `left_or` and `left_or_else` (and analogically for converting [`Some`] to
//! [`Right`]), but since the purpose of this conversion pattern is to be a generalization of
//! [`Option::unwrap_or`] ([`_else`]) supporting different types for different branches, whether
//! [`Some`] maps to [`Left`] or [`Right`] is irrelevant. As a result, the [`OptionEitherOr`]
//! extension trait provides [`either_or`] and [`either_or_else`].
//!
//! Providing conversions where it matters whether [`Some`] is mapped to [`Left`] or [`Right`] is
//! out of scope of this crate.
//!
//! [`Either`]: either::Either
//! [`Left`]: either::Either::Left
//! [`Right`]: either::Either::Right
//! [`_else`]: Option::unwrap_or_else
//! [`either_or`]: OptionEitherOr::either_or
//! [`either_or_else`]: OptionEitherOr::either_or_else

#![no_std]

use either::Either;

/// A trait that provides conversion to [`Either`]
pub trait OptionEitherOr {
    /// The type that will be converted into [`Either::Right`]
    type R;

    /// Convert `Self::R` into [`Either::Right`] or value returned from `f` into [`Either::Left`]
    /// if [`None`].
    fn either_or_else<F: FnOnce() -> L, L>(self, f: F) -> Either<L, Self::R>;

    /// Convert `Self::R` into [`Either::Right`] or `l` into [`Either::Left`] if [`None`].
    fn either_or<L>(self, l: L) -> Either<L, Self::R>
    where
        Self: Sized,
    {
        self.either_or_else(|| l)
    }
}

impl<T> OptionEitherOr for Option<T> {
    type R = T;

    fn either_or_else<F: FnOnce() -> L, L>(self, f: F) -> Either<L, Self::R> {
        match self {
            Some(l) => Either::Right(l),
            None => Either::Left(f()),
        }
    }
}

#[cfg(test)]
mod test {
    use either::Either;

    use crate::OptionEitherOr;

    #[test]
    fn converts_some() {
        let o = Some("a");
        let e = o.either_or(3);
        assert_eq!(e, Either::Right("a"))
    }

    #[test]
    fn converts_none() {
        let o = Option::<()>::None;
        let e = o.either_or(3);
        assert_eq!(e, Either::Left(3))
    }
}
