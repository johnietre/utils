// TODO: unwrap_err_or_*?
use core::ops::{Deref, DerefMut};
use core::{fmt, hint};
use PResult::*;

pub mod pio;

pub mod prelude {
    pub use super::PResult;
    pub use PResult::*;
}

/// Rust [`Result`] equivalent of a [`PResult`].
pub type PartialResult<T, E> = Result<(T, Option<E>), E>;

/// `PResult` is a type that represents a success ([`POk`]), failure ([`PErr`]), or partial success
/// and partial failure ([`PPartial`]).
#[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum PResult<T, E> {
    /// Contains the fully successful value.
    POk(T),
    /// Contains the partially successful value and the partially failed value.
    PPartial(T, E),
    /// Contains the fully failed value.
    PErr(E),
}

impl<T, E> PResult<T, E> {
    ///////////////////////////////////////////////////////////////////////////
    // Querying the contained values
    ///////////////////////////////////////////////////////////////////////////

    /// Returns `true` if the result is [`POk`] only.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::presult::prelude::*;
    ///
    /// let x: PResult<i32, i32> = POk(3);
    /// assert_eq!(x.is_ok(), true);
    ///
    /// let x: PResult<i32, i32> = PPartial(3, -3);
    /// assert_eq!(x.is_ok(), false);
    ///
    /// let x: PResult<i32, i32> = PErr(-3);
    /// assert_eq!(x.is_ok(), false);
    /// ```
    #[inline]
    pub const fn is_ok(&self) -> bool {
        matches!(*self, POk(_))
    }

    /// Returns `true` iff the result is [`POk`] and the value inside matches a predicate.
    #[inline]
    pub fn is_ok_and(self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            PErr(_) | PPartial(_, _) => false,
            POk(t) => f(t),
        }
    }

    /// Returns `true` if the result is [`PErr`] only.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::presult::prelude::*;
    ///
    /// let x: PResult<i32, i32> = POk(3);
    /// assert_eq!(x.is_partial(), false);
    ///
    /// let x: PResult<i32, i32> = PPartial(3, -3);
    /// assert_eq!(x.is_partial(), true);
    ///
    /// let x: PResult<i32, i32> = PErr(-3);
    /// assert_eq!(x.is_partial(), false);
    /// ```
    #[inline]
    pub const fn is_partial(&self) -> bool {
        matches!(*self, PPartial(_, _))
    }

    /// Returns `true` iff the result is [`PPartial`] and the value and error inside match a
    /// predicate.
    #[inline]
    pub fn is_partial_and(self, f: impl FnOnce(T, E) -> bool) -> bool {
        match self {
            PErr(_) | POk(_) => false,
            PPartial(t, e) => f(t, e),
        }
    }

    /// Returns `true` if the result is [`PErr`] only.
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::presult::prelude::*;
    ///
    /// let x: PResult<i32, i32> = POk(3);
    /// assert_eq!(x.is_err(), false);
    ///
    /// let x: PResult<i32, i32> = PPartial(3, -3);
    /// assert_eq!(x.is_err(), false);
    ///
    /// let x: PResult<i32, i32> = PErr(-3);
    /// assert_eq!(x.is_err(), true);
    /// ```
    #[inline]
    pub const fn is_err(&self) -> bool {
        matches!(*self, PErr(_))
    }

    /// Returns `true` iff the result is [`PErr`] and the value inside matches a predicate.
    #[inline]
    pub fn is_err_and(self, f: impl FnOnce(E) -> bool) -> bool {
        match self {
            POk(_) | PPartial(_, _) => false,
            PErr(e) => f(e),
        }
    }

    /// Returns `true` if the result is [`POk`] or [`PPartial`].
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::presult::prelude::*;
    ///
    /// let x: PResult<i32, i32> = POk(3);
    /// assert_eq!(x.has_ok(), true);
    ///
    /// let x: PResult<i32, i32> = PPartial(3, -3);
    /// assert_eq!(x.has_ok(), true);
    ///
    /// let x: PResult<i32, i32> = PErr(-3);
    /// assert_eq!(x.has_ok(), false);
    /// ```
    #[inline]
    pub const fn has_ok(&self) -> bool {
        matches!(*self, POk(_) | PPartial(_, _))
    }

    /// Returns `true` if the result is [`POk`] or [`PPartial`] and the value inside matches a
    /// predicate.
    #[inline]
    pub fn has_ok_and(self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            POk(t) | PPartial(t, _) => f(t),
            PErr(_) => false,
        }
    }

    /// Returns `true` if the result is [`PErr`] or [`PPartial`].
    ///
    /// # Examples
    ///
    /// ```
    /// use utils::presult::prelude::*;
    ///
    /// let x: PResult<i32, i32> = POk(3);
    /// assert_eq!(x.has_err(), false);
    ///
    /// let x: PResult<i32, i32> = PPartial(3, -3);
    /// assert_eq!(x.has_err(), true);
    ///
    /// let x: PResult<i32, i32> = PErr(-3);
    /// assert_eq!(x.has_err(), true);
    /// ```
    #[inline]
    pub const fn has_err(&self) -> bool {
        matches!(*self, PErr(_) | PPartial(_, _))
    }

    /// Returns `true` if the result is [`PErr`] or [`PPartial`] and the value inside matches a
    /// predicate.
    #[inline]
    pub fn has_err_and(self, f: impl FnOnce(E) -> bool) -> bool {
        match self {
            PErr(e) | PPartial(_, e) => f(e),
            POk(_) => false,
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    // Adapter for each variant
    ///////////////////////////////////////////////////////////////////////////

    /// Converts from `PResult<T, E>` to [`Option<T>`] iff it is [`POk`].
    /// Consumes self, discarding the error and possibly the value, depending on the variant.
    #[inline]
    pub fn ok(self) -> Option<T> {
        match self {
            POk(t) => Some(t),
            PPartial(_, _) | PErr(_) => None,
        }
    }

    /// Converts from `PResult<T, E>` to [`Option<(T, E)>`] iff it is [`PPartial`].
    /// Consumes self, discarding the error or value, depending on the variant.
    #[inline]
    pub fn partial(self) -> Option<(T, E)> {
        match self {
            PPartial(t, e) => Some((t, e)),
            POk(_) | PErr(_) => None,
        }
    }

    /// Converts from `PResult<T, E>` to [`Option<E>`] iff it is [`PErr`].
    /// Consumes self, discarding the value and possibly the error, depending on the variant.
    #[inline]
    pub fn err(self) -> Option<E> {
        match self {
            PErr(e) => Some(e),
            PPartial(_, _) | POk(_) => None,
        }
    }

    /// Converts from `PResult<T, E>` to [`Option<T>`] if it is [`POk`] or [`PPartial`].
    /// Consumes self, discarding the error, if any.
    #[inline]
    pub fn ok_any(self) -> Option<T> {
        match self {
            POk(t) | PPartial(t, _) => Some(t),
            PErr(_) => None,
        }
    }

    /// Converts from `PResult<T, E>` to [`Option<E>`] if it is [`PErr`] or [`PPartial`].
    /// Consumes self, discarding the value, if any.
    #[inline]
    pub fn err_any(self) -> Option<E> {
        match self {
            PPartial(_, e) | PErr(e) => Some(e),
            POk(_) => None,
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    // Adapter for working with references
    ///////////////////////////////////////////////////////////////////////////

    /// Converts from `&PResult<T, E>` to `PResult<&T, &E>`.
    ///
    /// Produces a new `PResult` containing a reference into the original, leaving the original in
    /// place.
    #[inline]
    pub const fn as_ref(&self) -> PResult<&T, &E> {
        match *self {
            POk(ref t) => POk(t),
            PPartial(ref t, ref e) => PPartial(t, e),
            PErr(ref e) => PErr(e),
        }
    }

    /// Converts from `&mut PResult<T, E>` to `PResult<&mut T, &mut E>`.
    #[inline]
    pub fn as_mut(&mut self) -> PResult<&mut T, &mut E> {
        match *self {
            POk(ref mut t) => POk(t),
            PPartial(ref mut t, ref mut e) => PPartial(t, e),
            PErr(ref mut e) => PErr(e),
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    // Transforming contained values
    ///////////////////////////////////////////////////////////////////////////

    /// Maps a `PResult<T, E>` to `PResult<U, E>` by applying a function to a contained [`POk`] or
    /// [`PPartial`] value, leaving the [`PErr`] value untouched.
    #[inline]
    pub fn map<U, F: FnOnce(T) -> U>(self, op: F) -> PResult<U, E> {
        match self {
            POk(t) => POk(op(t)),
            PPartial(t, e) => PPartial(op(t), e),
            PErr(e) => PErr(e),
        }
    }

    /// Returns the provided default (if [`PErr`] OR [`PPartial`]), or applies a function to the
    /// contained value (if [`POk`]).
    #[inline]
    pub fn map_or<U, F: FnOnce(T) -> U>(self, default: U, f: F) -> U {
        match self {
            POk(t) => f(t),
            PPartial(_, _) | PErr(_) => default,
        }
    }

    /// Returns the provided default (iff [`PErr`]), or applies a function to the contained value
    /// (if [`POk`] OR [`PPartial`]).
    #[inline]
    pub fn map_or_any<U, F: FnOnce(T) -> U>(self, default: U, f: F) -> U {
        match self {
            POk(t) | PPartial(t, _) => f(t),
            PErr(_) => default,
        }
    }

    /// Maps a `PResult<T, E>` to `U` by applying fallback function `default` (if [`PErr`] OR
    /// [`PPartial`]) to an error value, or function `f` (iff [`POk`]) to a success value.
    #[inline]
    pub fn map_or_else<U, D: FnOnce(E) -> U, F: FnOnce(T) -> U>(self, default: D, f: F) -> U {
        match self {
            POk(t) => f(t),
            PPartial(_, e) | PErr(e) => default(e),
        }
    }

    /// Maps a `PResult<T, E>` to `U` by applying fallback function `default` (iff [`PErr`]) to an
    /// error value, or function `f` (if [`POk`] OR [`PPartial`]) to a success value.
    #[inline]
    pub fn map_or_else_any<U, D: FnOnce(E) -> U, F: FnOnce(T) -> U>(self, default: D, f: F) -> U {
        match self {
            POk(t) | PPartial(t, _) => f(t),
            PErr(e) => default(e),
        }
    }

    /// Maps a `PResult<T, E>` to `PResult<T, F>` by applying a function to a contained [`PErr`] or
    /// [`PPartial`] value, leaving the [`POk`] value untouched.
    #[inline]
    pub fn map_err<F, O: FnOnce(E) -> F>(self, op: O) -> PResult<T, F> {
        match self {
            POk(t) => POk(t),
            PPartial(t, e) => PPartial(t, op(e)),
            PErr(e) => PErr(op(e)),
        }
    }

    /* TODO
    #[inline]
    pub fn join_inner_left(self, other: Self) -> Self {
        match (self, other) {
            (POk(t), POk(_)) => POk(t),
            (POk(t), PPartial(_, e)) | (POk(t), PErr(e)) => PPartial(t, e),

            (PPartial(_, e), POK(t)) => PPartial(t, e),
            (PPartial(t, e), PPartial(_, _)) => PPartial(t, e),
            (PPartial(t, _), PErr(e)) => PPartial(t, e),

            (PErr(e), POk(t)) | (PErr(e), PPartial(t, _)) => PPartial(t, e),
            (PErr(e), PErr(_)) => PErr(e),
        }
    }

    #[inline]
    pub fn join_inner_right(self, other: Self) -> Self {
        other.join_inner_left(self)
    }

    #[inline]
    pub fn join_left(self, other: Self) -> Self {
        match (self, other) {
            (POk(t), POk(_)) => POk(t),
            (POk(t), PPartial(_, e)) | (POk(t), PErr(e)) => PPartial(t, e),
            (PPartial(t, e), _) => PPartial(t, e),
            (PErr(e), POk(t)) | (PErr(e), PPartial(t, _)) => PPartial(t, e),
            (PErr(e), PErr(_)) => PErr(e),
        }
    }

    #[inline]
    pub fn join_right(self, other: Self) -> Self {
        other.join_left(self)
    }
    */

    #[inline]
    pub fn try_join_exclusive(self, other: Self) -> Result<Self, (Self, Self)> {
        match (self, other) {
            (POk(t), PErr(e)) | (PErr(e), POk(t)) => Ok(PPartial(t, e)),
            (this, other) => Err((this, other)),
        }
    }

    /// Calls a function with a reference to the contained value iff [`POk`].
    #[inline]
    pub fn inspect<F: FnOnce(&T)>(self, f: F) -> Self {
        if let POk(ref t) = self {
            f(t);
        }
        self
    }

    /// Calls a function with references to the contained value and error iff [`PPartial`].
    #[inline]
    pub fn inspect_partial<F: FnOnce(&T, &E)>(self, f: F) -> Self {
        if let PPartial(ref t, ref e) = self {
            f(t, e);
        }
        self
    }

    /// Calls a function with a reference to the contained error value iff [`PErr`].
    #[inline]
    pub fn inspect_err<F: FnOnce(&E)>(self, f: F) -> Self {
        if let PErr(ref e) = self {
            f(e);
        }
        self
    }

    /// Calls a function with a reference to the contained value if [`POk`] or [`PPartial`].
    #[inline]
    pub fn inspect_any<F: FnOnce(&T)>(self, f: F) -> Self {
        match self {
            POk(ref t) | PPartial(ref t, _) => f(t),
            PErr(_) => (),
        }
        self
    }

    /// Calls a function with a reference to the contained error value if [`PErr`] or [`PPartial`].
    #[inline]
    pub fn inspect_err_any<F: FnOnce(&E)>(self, f: F) -> Self {
        match self {
            POk(_) => (),
            PPartial(_, ref e) | PErr(ref e) => f(e),
        }
        self
    }

    /// Converts a [`PPartial`] to [`POk`], discarding the error, otherwise, does nothing.
    #[inline]
    pub fn partial_into_pok(self) -> Self {
        match self {
            POk(t) | PPartial(t, _) => POk(t),
            PErr(e) => PErr(e),
        }
    }

    /// Converts a [`PPartial`] to [`PErr`], discarding the value, otherwise, does nothing.
    #[inline]
    pub fn partial_into_perr(self) -> Self {
        match self {
            POk(t) => POk(t),
            PPartial(_, e) | PErr(e) => PErr(e),
        }
    }

    /// Converts the `PResult` into a [`Result`]. [`PPartial`] is mapped to [`Ok`], discarding
    /// the error.
    #[inline]
    pub fn into_res_ok(self) -> Result<T, E> {
        match self {
            POk(t) | PPartial(t, _) => Ok(t),
            PErr(e) => Err(e),
        }
    }

    /// Converts the `PResult` into a [`Result`]. [`PPartial`] is mapped to [`Err`], discarding
    /// the value.
    #[inline]
    pub fn into_res_err(self) -> Result<T, E> {
        match self {
            POk(t) => Ok(t),
            PPartial(_, e) | PErr(e) => Err(e),
        }
    }

    /// Converts from `PResult<T, E>` (or `&PResult<T, E>`) to
    /// `PResult<&<T as Deref>::Target, &E>`.
    /// Coerces the [`POk`] and [`PPartial`] variants of the original [`PResult`] via Deref and
    /// returns the new [`PResult`].
    #[inline]
    pub fn as_deref(&self) -> PResult<&T::Target, &E>
    where
        T: Deref,
    {
        self.as_ref().map(|t| t.deref())
    }

    /// Converts from `PResult<T, E>` (or `&mut PResult<T, E>`) to
    /// `PResult<&mut <T as DerefMut>::Target, &mut E>`.
    /// Coerces the [`POk`] and [`PPartial`] variants of the original [`PResult`] via DerefMut and
    /// returns the new [`PResult`].
    #[inline]
    pub fn as_deref_mut(&mut self) -> PResult<&mut T::Target, &mut E>
    where
        T: DerefMut,
    {
        self.as_mut().map(|t| t.deref_mut())
    }

    /// Convers from `PResult<T, E>` to `PResult<E, T>`.
    #[inline]
    pub fn swap_val_err(self) -> PResult<E, T> {
        match self {
            POk(t) => PErr(t),
            PPartial(t, e) => PPartial(e, t),
            PErr(e) => POk(e),
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    // Iterator constructors
    ///////////////////////////////////////////////////////////////////////////

    /// Returns an iterator over the possibly contained value iff the result is [`PResult::POk`],
    /// otherwise none.
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            inner: self.as_ref().ok(),
        }
    }

    /// Returns a mutable iterator over the possibly contained value iff the result is
    /// [`PResult::POk`], otherwise none.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            inner: self.as_mut().ok(),
        }
    }

    /// Returns an iterator over the possibly contained value and error iff the result is
    /// [`PResult::PPartial`], otherwise none.
    #[inline]
    pub fn iter_partial(&self) -> IntoIter<(&T, &E)> {
        IntoIter {
            inner: self.as_ref().partial(),
        }
    }

    /// Returns a mutable iterator over the possibly contained value and error iff the result is
    /// [`PResult::PPartial`], otherwise none.
    #[inline]
    pub fn iter_partial_mut(&mut self) -> IntoIter<(&mut T, &mut E)> {
        IntoIter {
            inner: self.as_mut().partial(),
        }
    }

    /// Returns an iterator over the possibly contained error value iff the result is
    /// [`PResult::PErr`], otherwise none.
    #[inline]
    pub fn iter_err(&self) -> Iter<'_, E> {
        Iter {
            inner: self.as_ref().err(),
        }
    }

    /// Returns a mutable iterator over the possibly contained error value iff the result is
    /// [`PResult::PErr`], otherwise none.
    #[inline]
    pub fn iter_err_mut(&mut self) -> IterMut<'_, E> {
        IterMut {
            inner: self.as_mut().err(),
        }
    }

    /// Returns an iterator over the possibly contained value if the result is [`PResult::POk`] or
    /// [`PResult::PPartial`], otherwise none.
    #[inline]
    pub fn iter_any(&self) -> Iter<'_, T> {
        Iter {
            inner: self.as_ref().ok_any(),
        }
    }

    /// Returns a mutable iterator over the possibly contained value if the result is
    /// [`PResult::POk`] or [`PResult::PPartial`], otherwise none.
    #[inline]
    pub fn iter_any_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            inner: self.as_mut().ok_any(),
        }
    }

    /// Returns an iterator over the possibly contained error value if the result is
    /// [`PResult::PErr`] or [`PResult::PPartial`], otherwise none.
    #[inline]
    pub fn iter_err_any(&self) -> Iter<'_, E> {
        Iter {
            inner: self.as_ref().err_any(),
        }
    }

    /// Returns a mutable iterator over the possibly contained error value if the result is
    /// [`PResult::PErr`] or [`PResult::PPartial`], otherwise none.
    #[inline]
    pub fn iter_err_any_mut(&mut self) -> IterMut<'_, E> {
        IterMut {
            inner: self.as_mut().err_any(),
        }
    }

    /// Returns a consuming iterator over the possibly contained success and error values.
    /// The iterator yields one value iff the result is [`PResult::PPartial`], otherwise none.
    #[inline]
    pub fn into_iter_partial(self) -> IntoIter<(T, E)> {
        IntoIter {
            inner: self.partial(),
        }
    }

    /// Returns a consuming iterator over the possibly contained error value.
    /// The iterator yields one value iff the result is [`PResult::PErr`], otherwise none.
    #[inline]
    pub fn into_iter_err(self) -> IntoIter<E> {
        IntoIter { inner: self.err() }
    }

    /// Returns a consuming iterator over the possibly contained value.
    /// The iterator yields one value if the result is [`PResult::POk`] or [`PResult::PPartial`],
    /// otherwise none.
    #[inline]
    pub fn into_iter_any(self) -> IntoIter<T> {
        IntoIter {
            inner: self.ok_any(),
        }
    }

    /// Returns a consuming iterator over the possibly contained error value.
    /// The iterator yields one value if the result is [`PResult::PErr`] or [`PResult::PPartial`],
    /// otherwise none.
    #[inline]
    pub fn into_iter_err_any(self) -> IntoIter<E> {
        IntoIter {
            inner: self.err_any(),
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    // Extract a value
    ///////////////////////////////////////////////////////////////////////////

    /// Returns the contained [`POk`] value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is a [`PErr`] OR [`PPartial`], with a panic message including the
    /// passed message, and the content of the error value.
    #[inline]
    pub fn expect(self, msg: &str) -> T
    where
        E: fmt::Debug,
    {
        match self {
            POk(t) => t,
            PPartial(_, e) | PErr(e) => unwrap_failed::<E>(msg, &e),
        }
    }

    /// Returns the contained [`POk`] value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is a [`PErr`] OR [`PPartial`], with a panic message provided by the
    /// error value.
    #[inline]
    pub fn unwrap(self) -> T
    where
        E: fmt::Debug,
    {
        match self {
            POk(t) => t,
            PPartial(_, e) => {
                unwrap_failed::<E>("called `PResult::unwrap()` on a `PPartial` value", &e)
            }
            PErr(e) => unwrap_failed::<E>("called `PResult::unwrap()` on a `PErr` value", &e),
        }
    }

    /// Returns the caonted [`POk`] valud or a default (if [`PErr`] OR [`PPartial`]).
    #[inline]
    pub fn unwrap_or_default(self) -> T
    where
        T: Default,
    {
        match self {
            POk(t) => t,
            PPartial(_, _) | PErr(_) => Default::default(),
        }
    }

    /// Returns the contained [`PPartial`] success and error values, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is a [`POk`] OR [`PErr`], with a panic message including the passed
    /// message, and the content of the error value.
    #[inline]
    pub fn expect_partial(self, msg: &str) -> (T, E)
    where
        T: fmt::Debug,
        E: fmt::Debug,
    {
        match self {
            PPartial(t, e) => (t, e),
            POk(t) => unwrap_failed::<T>(msg, &t),
            PErr(e) => unwrap_failed::<E>(msg, &e),
        }
    }

    /// Returns the contained [`PPartial`] success and error values, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is a [`POk`] OR [`PErr`], with a panic message provided by the error
    /// value.
    #[inline]
    pub fn unwrap_partial(self) -> (T, E)
    where
        T: fmt::Debug,
        E: fmt::Debug,
    {
        match self {
            PPartial(t, e) => (t, e),
            POk(t) => unwrap_failed::<T>("called `PResult::unwrap_partial()` on a `POk` value", &t),
            PErr(e) => {
                unwrap_failed::<E>("called `PResult::unwrap_partial()` on a `PErr` value", &e)
            }
        }
    }

    /// Returns the contained [`PERr`] value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is a [`POk`] OR [`PPartial`], with a panic message including the
    /// passed message, and the content of the value.
    #[inline]
    pub fn expect_err(self, msg: &str) -> E
    where
        T: fmt::Debug,
    {
        match self {
            PErr(e) => e,
            PPartial(t, _) | POk(t) => unwrap_failed::<T>(msg, &t),
        }
    }

    /// Returns the contained [`PErr`] value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is a [`POk`] OR [`PPartial`], with a panic message provided by the
    /// value.
    #[inline]
    pub fn unwrap_err(self) -> E
    where
        T: fmt::Debug,
    {
        match self {
            PErr(e) => e,
            POk(t) => unwrap_failed::<T>("called `PResult::unwrap_err()` on a `POk` value", &t),
            PPartial(t, _) => {
                unwrap_failed::<T>("called `PResult::unwrap_err()` on a `PPartial` value", &t)
            }
        }
    }

    /// Same as [`PResult::expect`] but also accepts [`PPartial`].
    ///
    /// # Panics
    ///
    /// See [`PResult::expect`].
    #[inline]
    pub fn expect_any(self, msg: &str) -> T
    where
        E: fmt::Debug,
    {
        match self {
            POk(t) | PPartial(t, _) => t,
            PErr(e) => unwrap_failed::<E>(msg, &e),
        }
    }

    /// Same as [`PResult::unwrap`] but also accepts [`PPartial`].
    ///
    /// # Panics
    ///
    /// See [`PResult::unwrap`].
    #[inline]
    pub fn unwrap_any(self) -> T
    where
        E: fmt::Debug,
    {
        match self {
            POk(t) | PPartial(t, _) => t,
            PErr(e) => unwrap_failed::<E>("called `PResult::unwrap_any()` on a `PErr` value", &e),
        }
    }

    /// Same as [`PResult::unwrap_or_default`] but also accepts [`PPartial`].
    ///
    /// # Panics
    ///
    /// See [`PResult::unwrap_or_default`].
    #[inline]
    pub fn unwrap_any_or_default(self) -> T
    where
        T: Default,
    {
        match self {
            POk(t) | PPartial(t, _) => t,
            PErr(_) => Default::default(),
        }
    }

    /// Same as [`PResult::expect_err`] but also accepts [`PPartial`].
    ///
    /// # Panics
    ///
    /// See [`PResult::expect_err`].
    #[inline]
    pub fn expect_err_any(self, msg: &str) -> E
    where
        T: fmt::Debug,
    {
        match self {
            PErr(e) | PPartial(_, e) => e,
            POk(t) => unwrap_failed::<T>(msg, &t),
        }
    }

    /// Same as [`PResult::unwrap_err`] but also accepts [`PPartial`].
    ///
    /// # Panics
    ///
    /// See [`PResult::unwrap_err`].
    #[inline]
    pub fn unwrap_err_any(self) -> E
    where
        T: fmt::Debug,
    {
        match self {
            PErr(e) | PPartial(_, e) => e,
            POk(t) => unwrap_failed::<T>("called `PResult::unwrap_err_any()` on a `POk` value", &t),
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    // Boolean operations on the values, eager and lazy
    ///////////////////////////////////////////////////////////////////////////

    // NOTE: No `and`, `and_then`, `or`, or `or_else` due to the partial needing to be changed.

    /*
    #[inline]
    pub fn and<U>(self, pres: PResult<U, E>) -> PResult<U, E> {
        match self {
            POk(_) => pres,
            PPartial(t, e) => PPartial(t, e),
            PErr(e) => PErr(e),
            //PPartial(_, _) | PErr(_) => self,
        }
    }

    #[inline]
    pub fn and_then<U, F: FnOnce(T) -> PResult<U, E>>(self, op: F) -> PResult<U, E> {
        match self {
            POk(t) => op(t),
            PPartial(t, e) => PPartial(op(t), e),
            PErr(e) => PErr(e),
            //PPartial(_, _) | PErr(_) => self,
        }
    }

    #[inline]
    pub fn or<F>(self, pres: PResult<T, F>) -> PResult<T, F> {
        match self {
            POk(t) => POk(t),
            PPartial(t, e) => PPartial(t, e),
            //POk(_) | PPartial(_, _) => self,
            PErr(_) => pres,
        }
    }

    #[inline]
    pub fn or_else<F, O: FnOnce(E) -> PResult<T, F>>(self, op: O) -> PResult<T, F> {
        match self {
            POk(t) => POk(t),
            PPartial(t, e) => PPartial(t, op(e)),
            PErr(e) => op(e),
        }
    }
    */

    /// Returns `pres` if the result is [`POk`] or [`PPartial`], otherwise, returns the [`PErr`]
    /// value of `self`.
    #[inline]
    pub fn and_any<U>(self, pres: PResult<U, E>) -> PResult<U, E> {
        match self {
            POk(_) | PPartial(_, _) => pres,
            PErr(e) => PErr(e),
        }
    }

    /// Calls `op` if the result is [`POk`] or [`PPartial`], otherwise returns the [`PErr`] value
    /// of self.
    #[inline]
    pub fn and_then_any<U, F: FnOnce(T) -> PResult<U, E>>(self, op: F) -> PResult<U, E> {
        match self {
            POk(t) | PPartial(t, _) => op(t),
            PErr(e) => PErr(e),
        }
    }

    /// Returns `pres` if the result is [`PErr`] or [`PPartial`], otherwise, returns the [`POk`]
    /// value of `self`.
    #[inline]
    pub fn or_any<F>(self, pres: PResult<T, F>) -> PResult<T, F> {
        match self {
            POk(t) => POk(t),
            PPartial(_, _) | PErr(_) => pres,
        }
    }

    /// Calls `op` if the result is [`PErr`] or [`PPartial`], otherwise returns the [`POk`] value
    /// of self.
    #[inline]
    pub fn or_else_any<F, O: FnOnce(E) -> PResult<T, F>>(self, op: O) -> PResult<T, F> {
        match self {
            POk(t) => POk(t),
            PPartial(_, e) | PErr(e) => op(e),
        }
    }

    /// Returns the contained [`POk`] value or a provided default, if [`PErr`] OR [`PPartial`].
    #[inline]
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            POk(t) => t,
            PPartial(_, _) | PErr(_) => default,
        }
    }

    /// Returns the contained [`POk`] value or computes it from a closure, if [`PErr`] OR
    /// [`PPartial`].
    #[inline]
    pub fn unwrap_or_else<F: FnOnce(E) -> T>(self, op: F) -> T {
        match self {
            POk(t) => t,
            PPartial(_, e) | PErr(e) => op(e),
        }
    }

    /// Returns the contained [`POk`] OR [`PPartial`] value or a provided default.
    #[inline]
    pub fn unwrap_any_or(self, default: T) -> T {
        match self {
            POk(t) | PPartial(t, _) => t,
            PErr(_) => default,
        }
    }

    /// Returns the contained [`POk`] OR [`PPartial`] value or computes it from a closure.
    #[inline]
    pub fn unwrap_any_or_else<F: FnOnce(E) -> T>(self, op: F) -> T {
        match self {
            POk(t) | PPartial(t, _) => t,
            PErr(e) => op(e),
        }
    }

    /// Returns the contained [`POk`] value, consuming the `self` value, without checking that the
    /// value is not [`PErr`] or [`PPartial`].
    ///
    /// # Safety
    ///
    /// Calling this method on [`PErr`] or [`PPartial`] is *[undefined behavior]*.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    #[inline]
    pub unsafe fn unwrap_unchecked(self) -> T {
        debug_assert!(self.is_ok());
        match self {
            POk(t) => t,
            // SAFETY: the safety contract must be upheld by the caller.
            PPartial(_, _) | PErr(_) => unsafe { hint::unreachable_unchecked() },
        }
    }

    /// Returns the contained [`PPartial`] value, consuming the `self` value, without checking that
    /// the value is not [`POk`] or [`PErr`].
    ///
    /// # Safety
    ///
    /// Calling this method on [`POk`] or [`PErr`] is *[undefined behavior]*.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    #[inline]
    pub unsafe fn unwrap_partial_unchecked(self) -> (T, E) {
        debug_assert!(self.is_partial());
        match self {
            PPartial(t, e) => (t, e),
            // SAFETY: the safety contract must be upheld by the caller.
            POk(_) | PErr(_) => unsafe { hint::unreachable_unchecked() },
        }
    }

    /// Returns the contained [`PErr`] value, consuming the `self` value, without checking that the
    /// value is not [`POk`] or [`PPartial`].
    ///
    /// # Safety
    ///
    /// Calling this method on [`POk`] or [`PPartial`] is *[undefined behavior]*.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    #[inline]
    pub unsafe fn unwrap_err_unchecked(self) -> E {
        debug_assert!(self.is_err());
        match self {
            PErr(e) => e,
            // SAFETY: the safety contract must be upheld by the caller.
            PPartial(_, _) | POk(_) => unsafe { hint::unreachable_unchecked() },
        }
    }

    /// Returns the contained [`POk`] or [`PPartial`] error value, consuming the `self` value,
    /// without checking that the value is not [`PErr`].
    ///
    /// # Safety
    ///
    /// Calling this method on [`PErr`] is *[undefined behavior]*.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    #[inline]
    pub unsafe fn unwrap_any_unchecked(self) -> T {
        debug_assert!(!self.is_err());
        match self {
            POk(t) | PPartial(t, _) => t,
            // SAFETY: the safety contract must be upheld by the caller.
            PErr(_) => unsafe { hint::unreachable_unchecked() },
        }
    }

    /// Returns the contained [`PErr`] or [`PPartial`] error value, consuming the `self` value,
    /// without checking that the value is not [`POk`].
    ///
    /// # Safety
    ///
    /// Calling this method on [`POk`] is *[undefined behavior]*.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    #[inline]
    pub unsafe fn unwrap_err_any_unchecked(self) -> E {
        debug_assert!(!self.is_ok());
        match self {
            PErr(e) | PPartial(_, e) => e,
            // SAFETY: the safety contract must be upheld by the caller.
            POk(_) => unsafe { hint::unreachable_unchecked() },
        }
    }
}

impl<T, E> PResult<&T, E> {
    /// Maps a `PResult<&T, E>` to a `PResult<T, E>` by copying the contents of the `POk` and
    /// `PPartial` variants.
    #[inline]
    pub fn copied(self) -> PResult<T, E>
    where
        T: Copy,
    {
        self.map(|&t| t)
    }

    /// Maps a `PResult<&T, E>` to a `PResult<T, E>` by cloning the contents of the `POk` and
    /// `PPartial` variants.
    #[inline]
    pub fn cloned(self) -> PResult<T, E>
    where
        T: Clone,
    {
        self.map(|t| t.clone())
    }
}

impl<T, E> PResult<&mut T, E> {
    /// Maps a `PResult<&mut T, E>` to a `PResult<T, E>` by copying the contents of the `POk` and
    /// `PPartial` variants.
    #[inline]
    pub fn copied(self) -> PResult<T, E>
    where
        T: Copy,
    {
        self.map(|&mut t| t)
    }

    /// Maps a `PResult<&mut T, E>` to a `PResult<T, E>` by cloning the contents of the `POk` and
    /// `PPartial` variants.
    #[inline]
    pub fn cloned(self) -> PResult<T, E>
    where
        T: Clone,
    {
        self.map(|t| t.clone())
    }
}

impl<T, E> PResult<Option<T>, E> {
    /// Transpose a `PResult` of an `Option` into an `Option` of a `PResult`.
    /// `POk(None)` is mapped to `None`. All else are mapped to their respective `Some(_)`.
    /// `PPartial(None, _)` is mapped to `Some(PErr(_))`.
    #[inline]
    pub fn transpose(self) -> Option<PResult<T, E>> {
        match self {
            POk(Some(t)) => Some(POk(t)),
            POk(None) => None,
            PPartial(Some(t), e) => Some(PPartial(t, e)),
            PPartial(None, e) | PErr(e) => Some(PErr(e)),
        }
    }
}

impl<T, E> PResult<Result<T, E>, E> {
    /// Converts from `PResult<Result<T, E>, E> to `PResult<T, E>`.
    #[inline]
    pub fn flatten(self) -> PResult<T, E> {
        match self {
            POk(res) => match res {
                Ok(t) => POk(t),
                Err(e) => PErr(e),
            },
            PPartial(res, e) => match res {
                Ok(t) => PPartial(t, e),
                // TODO: Other e?
                Err(_e) => PErr(e),
            },
            PErr(e) => PErr(e),
        }
        //self.and_then_any(std::convert::identity)
    }
}

impl<T, E> PResult<PResult<T, E>, E> {
    /// Converts from `PResult<PResult<T, E>, E> to `PResult<T, E>`.
    #[inline]
    pub fn flatten(self) -> PResult<T, E> {
        match self {
            POk(pres) => match pres {
                POk(t) => POk(t),
                PPartial(t, e) => PPartial(t, e),
                PErr(e) => PErr(e),
            },
            PPartial(pres, e) => match pres {
                POk(t) => PPartial(t, e),
                // TODO: Other e?
                PPartial(t, _e) => PPartial(t, e),
                PErr(e) => PErr(e),
            },
            PErr(e) => PErr(e),
        }
        //self.and_then_any(std::convert::identity)
    }
}

//#[cfg(not(feature = "panic_immediate_abort"))]
#[inline]
#[cold]
#[track_caller]
fn unwrap_failed<T>(msg: &str, error: &dyn fmt::Debug) -> ! {
    panic!("{msg}: {error:?}")
}

/*
#[cfg(feature = "panic_immediate_abort")]
#[inline]
#[cold]
#[track_caller]
fn unwrap_failed<T>(_msg: &str, _error: &T) -> ! {
    panic!()
}
*/

///////////////////////////////////////////////////////////////////////////
// Trait implementations
///////////////////////////////////////////////////////////////////////////

impl<T, E> Clone for PResult<T, E>
where
    T: Clone,
    E: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        match self {
            POk(t) => POk(t.clone()),
            PPartial(t, e) => PPartial(t.clone(), e.clone()),
            PErr(e) => PErr(e.clone()),
        }
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        match (self, source) {
            (POk(to), POk(from)) => to.clone_from(from),
            (PPartial(tot, toe), PPartial(fromt, frome)) => {
                tot.clone_from(fromt);
                toe.clone_from(frome);
            }
            (PErr(to), PErr(from)) => to.clone_from(from),
            (to, from) => *to = from.clone(),
        }
    }
}

impl<T, E> From<PResult<T, E>> for PartialResult<T, E> {
    fn from(pres: PResult<T, E>) -> Self {
        match pres {
            POk(t) => Ok((t, None)),
            PPartial(t, e) => Ok((t, Some(e))),
            PErr(e) => Err(e),
        }
    }
}

impl<T, E> From<Result<T, E>> for PResult<T, E> {
    fn from(res: Result<T, E>) -> Self {
        match res {
            Ok(t) => POk(t),
            Err(e) => PErr(e),
        }
    }
}

impl<T, E> From<PartialResult<T, E>> for PResult<T, E> {
    fn from(res: PartialResult<T, E>) -> Self {
        match res {
            Ok((t, None)) => POk(t),
            Ok((t, Some(e))) => PPartial(t, e),
            Err(e) => PErr(e),
        }
    }
}

impl<T, E> IntoIterator for PResult<T, E> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Returns a consuming iterator over the possibly contained value iff the result is
    /// [`PResult::POk`], otherwise none.
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter { inner: self.ok() }
    }
}

impl<'a, T, E> IntoIterator for &'a PResult<T, E> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, E> IntoIterator for &'a mut PResult<T, E> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/*
impl<T, E> crate::die::OrDie<T> for PResult<T, E> {
    #[inline]
    fn or_die_code_msg(self, code: i32, msg: &str) -> T {
        match self {
            POk(t) => t,
            PPartial(_, e) | PErr(e) => {
                eprintln!("{msg}: {e:?}");
                exit(code)
            }
        }
    }

    #[inline]
    fn or_die(self) -> T {
        self.or_die_code(1)
    }

    #[inline]
    fn or_die_code(self, code: i32) -> T {
        match self {
            POk(t) => t,
            PPartial(_, e) => {
                eprintln!("called `PResult::or_die()` on an `PPartial` value");
                exit(code)
            }
            PErr(e) => {
                eprintln!("called `PResult::or_die()` on an `PErr` value");
                exit(code)
            }
        }
    }

    #[inline]
    fn or_die_msg(self, msg: &str) -> T {
        self.or_die_code_msg(1, msg)
    }
}
*/

///////////////////////////////////////////////////////////////////////////
// The PResult Iterators
///////////////////////////////////////////////////////////////////////////

pub struct Iter<'a, T: 'a> {
    inner: Option<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.take()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = if self.inner.is_some() { 1 } else { 0 };
        (n, Some(n))
    }
}

pub struct IterMut<'a, T: 'a> {
    inner: Option<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.take()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = if self.inner.is_some() { 1 } else { 0 };
        (n, Some(n))
    }
}

#[derive(Clone, Debug)]
pub struct IntoIter<T> {
    inner: Option<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.take()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = if self.inner.is_some() { 1 } else { 0 };
        (n, Some(n))
    }
}

// TODO: FromIterator
// TODO: The rest

#[cfg(test)]
mod test {
    use super::prelude::*;

    const POK: PResult<i32, i32> = POk(123);
    const PPART: PResult<i32, i32> = PPartial(123, -123);
    const PERR: PResult<i32, i32> = PErr(-123);

    #[test]
    fn queries() {
        let ppart = PPartial(123, -123);
        let (pok, perr) = (ppart.partial_into_pok(), ppart.partial_into_perr());

        assert!(pok.is_ok());
        assert!(!pok.is_ok_and(|_| false));
        assert!(!pok.is_partial());
        assert!(!pok.is_partial_and(|_, _| true));
        assert!(!pok.is_err());
        assert!(!pok.is_err_and(|_| true));
        assert!(pok.has_ok());
        assert!(pok.has_ok_and(|_| true));
        assert!(!pok.has_err());
        assert!(!pok.has_err_and(|_| true));

        assert!(!ppart.is_ok());
        assert!(!ppart.is_ok_and(|_| true));
        assert!(ppart.is_partial());
        assert!(!ppart.is_partial_and(|_, _| false));
        assert!(!ppart.is_err());
        assert!(!ppart.is_err_and(|_| true));
        assert!(ppart.has_ok());
        assert!(ppart.has_ok_and(|_| true));
        assert!(ppart.has_err());
        assert!(ppart.has_err_and(|_| true));

        assert!(!perr.is_ok());
        assert!(!perr.is_ok_and(|_| true));
        assert!(!perr.is_partial());
        assert!(!perr.is_partial_and(|_, _| true));
        assert!(perr.is_err());
        assert!(!perr.is_err_and(|_| false));
        assert!(!perr.has_ok());
        assert!(!perr.has_ok_and(|_| true));
        assert!(perr.has_err());
        assert!(perr.has_err_and(|_| true));
    }

    #[test]
    fn variant_adapters() {
        let ppart = PPartial(123, -123);
        let (pok, perr) = (ppart.partial_into_pok(), ppart.partial_into_perr());

        assert_eq!(pok.ok(), Some(123));
        assert_eq!(pok.partial(), None);
        assert_eq!(pok.err(), None);
        assert_eq!(pok.ok_any(), Some(123));
        assert_eq!(pok.err_any(), None);

        assert_eq!(perr.ok(), None);
        assert_eq!(perr.partial(), None);
        assert_eq!(perr.err(), Some(-123));
        assert_eq!(perr.ok_any(), None);
        assert_eq!(perr.err_any(), Some(-123));

        assert_eq!(ppart.ok(), None);
        assert_eq!(ppart.partial(), Some((123, -123)));
        assert_eq!(ppart.err(), None);
        assert_eq!(ppart.ok_any(), Some(123));
        assert_eq!(ppart.err_any(), Some(-123));
    }

    mod transformations {
        use super::*;

        #[test]
        fn map() {
            let ppart = PPartial(123, -123);
            let (pok, perr) = (ppart.partial_into_pok(), ppart.partial_into_perr());

            assert_eq!(pok.map(|_| 456), POk(456));
            assert_eq!(pok.map_or(-456, |_| 456), 456);
            assert_eq!(pok.map_or_any(-456, |_| 456), 456);
            assert_eq!(pok.map_or_else(|_| -456, |_| 456), 456);
            assert_eq!(pok.map_or_else_any(|_| -456, |_| 456), 456);
            assert_eq!(pok.map_err(|_| -456), POk(123));

            assert_eq!(ppart.map(|_| 456), PPartial(456, -123));
            assert_eq!(ppart.map_or(-456, |_| 456), -456);
            assert_eq!(ppart.map_or_any(-456, |_| 456), 456);
            assert_eq!(ppart.map_or_else(|_| -456, |_| 456), -456);
            assert_eq!(ppart.map_or_else_any(|_| -456, |_| 456), 456);
            assert_eq!(ppart.map_err(|_| -456), PPartial(123, -456));

            assert_eq!(perr.map(|_| 456), perr);
            assert_eq!(perr.map_or(-456, |_| 456), -456);
            assert_eq!(perr.map_or_any(-456, |_| 456), -456);
            assert_eq!(perr.map_or_else(|_| -456, |_| 456), -456);
            assert_eq!(perr.map_or_else_any(|_| -456, |_| 456), -456);
            assert_eq!(perr.map_err(|_| -456), PErr(-456));
        }

        #[test]
        fn join() {
            let ppart1 = PPartial(123, -123);
            let (pok1, perr1) = (ppart1.partial_into_pok(), ppart1.partial_into_perr());
            let ppart2 = PPartial(456, -456);
            let (pok2, perr2) = (ppart2.partial_into_pok(), ppart2.partial_into_perr());

            /*
            // POk
            assert_eq!(pok1.join_left(pok2), POk(123));
            assert_eq!(pok1.join_left(ppart2), PPartial(123, -456));
            assert_eq!(pok1.join_left(perr2), PPartial(123, -456));

            assert_eq!(pok1.join_right(pok2), POk(456));
            assert_eq!(pok1.join_right(ppart2), ppart2);
            assert_eq!(pok1.join_right(perr2), PPartial(123, -456));

            assert_eq!(pok1.join_inner_left(pok2), POk(123));
            assert_eq!(pok1.join_inner_left(ppart2), PPartial(123, -456));
            assert_eq!(pok1.join_inner_left(perr2), PPartial(123, -456));

            assert_eq!(pok1.join_inner_right(pok2), POk(456));
            assert_eq!(pok1.join_inner_right(ppart2), PPartial(456, -456));
            assert_eq!(pok1.join_inner_right(perr2), PPartial(123, -456));

            // PPartial
            assert_eq!(ppart1.join_left(pok2), ppart1);
            assert_eq!(ppart1.join_left(ppart2), ppart1);
            assert_eq!(ppart1.join_left(perr2), ppart1);

            assert_eq!(ppart1.join_right(pok2), POk(456, -123));
            assert_eq!(ppart1.join_right(ppart2), ppart2);
            assert_eq!(ppart1.join_right(perr2), PPart(123, -456));

            // PErr
            assert_eq!(perr1.join_left(pok2), PPartial(456, -123));
            assert_eq!(perr1.join_left(ppart2), PPartial(456, -123));
            assert_eq!(perr1.join_left(perr2), PErr(-123));

            assert_eq!(perr1.join_right(pok2), PPartial(456, -123));
            assert_eq!(perr1.join_right(ppart2), ppart2);
            assert_eq!(perr1.join_right(perr2), PErr(-456));
            */

            assert_eq!(pok1.try_join_exclusive(pok2), Err((pok1, pok2)));
            assert_eq!(pok1.try_join_exclusive(ppart2), Err((pok1, ppart2)));
            assert_eq!(pok1.try_join_exclusive(perr2), Ok(PPartial(123, -456)));

            assert_eq!(ppart1.try_join_exclusive(pok2), Err((ppart1, pok2)));
            assert_eq!(ppart1.try_join_exclusive(ppart2), Err((ppart1, ppart2)));
            assert_eq!(ppart1.try_join_exclusive(perr2), Err((ppart1, perr2)));

            assert_eq!(perr1.try_join_exclusive(pok2), Ok(PPartial(456, -123)));
            assert_eq!(perr1.try_join_exclusive(ppart2), Err((perr1, ppart2)));
            assert_eq!(perr1.try_join_exclusive(perr2), Err((perr1, perr2)));
        }

        #[test]
        fn inspect() {
            let ppart = PPartial(123, -123);
            let (pok, perr) = (ppart.partial_into_pok(), ppart.partial_into_perr());

            let mut vals = Vec::new();
            pok.inspect(|&t| vals.push(t))
                .inspect_partial(|&t, &e| {
                    vals.push(t);
                    vals.push(e);
                })
                .inspect_err(|&e| vals.push(e))
                .inspect_any(|&t| vals.push(t))
                .inspect_err_any(|&e| vals.push(e));
            assert_eq!(vals, vec![123, 123]);

            let mut vals = Vec::new();
            ppart
                .inspect(|&t| vals.push(t))
                .inspect_partial(|&t, &e| {
                    vals.push(t);
                    vals.push(e);
                })
                .inspect_err(|&e| vals.push(e))
                .inspect_any(|&t| vals.push(t))
                .inspect_err_any(|&e| vals.push(e));
            assert_eq!(vals, vec![123, -123, 123, -123]);

            let mut vals = Vec::new();
            perr.inspect(|&t| vals.push(t))
                .inspect_partial(|&t, &e| {
                    vals.push(t);
                    vals.push(e);
                })
                .inspect_err(|&e| vals.push(e))
                .inspect_any(|&t| vals.push(t))
                .inspect_err_any(|&e| vals.push(e));
            assert_eq!(vals, vec![-123, -123]);
        }

        #[test]
        fn into() {
            let ppart = PPartial(123, -123);
            let (pok, perr) = (ppart.partial_into_pok(), ppart.partial_into_perr());

            assert_eq!(pok.partial_into_pok(), POk(123));
            assert_eq!(pok.partial_into_perr(), POk(123));
            assert_eq!(pok.into_res_err(), Ok(123));
            assert_eq!(pok.into_res_ok(), Ok(123));

            assert_eq!(ppart.partial_into_pok(), POk(123));
            assert_eq!(ppart.partial_into_perr(), PErr(-123));
            assert_eq!(ppart.into_res_err(), Err(-123));
            assert_eq!(ppart.into_res_ok(), Ok(123));

            assert_eq!(perr.partial_into_pok(), PErr(-123));
            assert_eq!(perr.partial_into_perr(), PErr(-123));
            assert_eq!(perr.into_res_err(), Err(-123));
            assert_eq!(perr.into_res_ok(), Err(-123));
        }
    }

    mod iters {
        use super::*;

        const EMPTY: Vec<i32> = Vec::new();
        const EMPTY_REF: Vec<&i32> = Vec::new();
        const EMPTY_MUT: Vec<&mut i32> = Vec::new();

        const PEMPTY: Vec<(i32, i32)> = Vec::new();
        const PEMPTY_REF: Vec<(&i32, &i32)> = Vec::new();
        const PEMPTY_MUT: Vec<(&mut i32, &mut i32)> = Vec::new();

        #[test]
        fn pok() {
            let mut pok = POK;
            assert_eq!(pok.iter().collect::<Vec<_>>(), vec![&123]);
            assert_eq!(pok.iter_partial().collect::<Vec<_>>(), PEMPTY_REF);
            assert_eq!(pok.iter_err().collect::<Vec<_>>(), EMPTY_REF);
            assert_eq!(pok.iter_any().collect::<Vec<_>>(), vec![&123]);
            assert_eq!(pok.iter_err_any().collect::<Vec<_>>(), EMPTY_REF);

            assert_eq!(pok.iter_mut().collect::<Vec<_>>(), vec![&mut 123]);
            assert_eq!(pok.iter_partial_mut().collect::<Vec<_>>(), PEMPTY_MUT);
            assert_eq!(pok.iter_err_mut().collect::<Vec<_>>(), EMPTY_MUT);
            assert_eq!(pok.iter_any_mut().collect::<Vec<_>>(), vec![&mut 123]);
            assert_eq!(pok.iter_err_any_mut().collect::<Vec<_>>(), EMPTY_MUT);

            assert_eq!(pok.into_iter().collect::<Vec<_>>(), vec![123]);
            assert_eq!(pok.into_iter_partial().collect::<Vec<_>>(), PEMPTY);
            assert_eq!(pok.into_iter_err().collect::<Vec<_>>(), EMPTY);
            assert_eq!(pok.into_iter_any().collect::<Vec<_>>(), vec![123]);
            assert_eq!(pok.into_iter_err_any().collect::<Vec<_>>(), EMPTY);
        }

        #[test]
        fn ppartial() {
            let mut ppart = PPART;
            assert_eq!(ppart.iter().collect::<Vec<_>>(), EMPTY_REF);
            assert_eq!(
                ppart.iter_partial().collect::<Vec<_>>(),
                vec![(&123, &-123)]
            );
            assert_eq!(ppart.iter_err().collect::<Vec<_>>(), EMPTY_REF);
            assert_eq!(ppart.iter_any().collect::<Vec<_>>(), vec![&123]);
            assert_eq!(ppart.iter_err_any().collect::<Vec<_>>(), vec![&-123]);

            assert_eq!(ppart.iter_mut().collect::<Vec<_>>(), EMPTY_MUT);
            assert_eq!(
                ppart.iter_partial_mut().collect::<Vec<_>>(),
                vec![(&mut 123, &mut -123)]
            );
            assert_eq!(ppart.iter_err_mut().collect::<Vec<_>>(), EMPTY_MUT);
            assert_eq!(ppart.iter_any_mut().collect::<Vec<_>>(), vec![&mut 123]);
            assert_eq!(
                ppart.iter_err_any_mut().collect::<Vec<_>>(),
                vec![&mut -123]
            );

            assert_eq!(ppart.into_iter().collect::<Vec<_>>(), EMPTY);
            assert_eq!(
                ppart.into_iter_partial().collect::<Vec<_>>(),
                vec![(123, -123)]
            );
            assert_eq!(ppart.into_iter_err().collect::<Vec<_>>(), EMPTY);
            assert_eq!(ppart.into_iter_any().collect::<Vec<_>>(), vec![123]);
            assert_eq!(ppart.into_iter_err_any().collect::<Vec<_>>(), vec![-123]);
        }

        #[test]
        fn perr() {
            let mut perr = PERR;
            assert_eq!(perr.iter().collect::<Vec<_>>(), EMPTY_REF);
            assert_eq!(perr.iter_partial().collect::<Vec<_>>(), PEMPTY_REF);
            assert_eq!(perr.iter_err().collect::<Vec<_>>(), vec![&-123]);
            assert_eq!(perr.iter_any().collect::<Vec<_>>(), EMPTY_REF);
            assert_eq!(perr.iter_err_any().collect::<Vec<_>>(), vec![&-123]);

            assert_eq!(perr.iter_mut().collect::<Vec<_>>(), EMPTY_MUT);
            assert_eq!(perr.iter_partial_mut().collect::<Vec<_>>(), PEMPTY_MUT);
            assert_eq!(perr.iter_err_mut().collect::<Vec<_>>(), vec![&mut -123]);
            assert_eq!(perr.iter_any_mut().collect::<Vec<_>>(), EMPTY_MUT);
            assert_eq!(perr.iter_err_any_mut().collect::<Vec<_>>(), vec![&mut -123]);

            assert_eq!(perr.into_iter().collect::<Vec<_>>(), EMPTY);
            assert_eq!(perr.into_iter_partial().collect::<Vec<_>>(), PEMPTY);
            assert_eq!(perr.into_iter_err().collect::<Vec<_>>(), vec![-123]);
            assert_eq!(perr.into_iter_any().collect::<Vec<_>>(), EMPTY);
            assert_eq!(perr.into_iter_err_any().collect::<Vec<_>>(), vec![-123]);
        }
    }

    mod extractions {
        // TODO
    }

    mod boolean_ops {
        use super::*;
        #[test]
        fn and_or() {
            let ppart1 = PPART;
            let (pok1, perr1) = (ppart1.partial_into_pok(), ppart1.partial_into_perr());
            let ppart2 = PPartial(456, -456);
            let (pok2, perr2) = (ppart2.partial_into_pok(), ppart2.partial_into_perr());

            // POk
            assert_eq!(pok1.and_any(pok2), pok2);
            assert_eq!(pok1.and_any(ppart2), ppart2);
            assert_eq!(pok1.and_any(perr2), perr2);

            assert_eq!(pok1.and_then_any(|_| pok2), pok2);
            assert_eq!(pok1.and_then_any(|_| ppart2), ppart2);
            assert_eq!(pok1.and_then_any(|_| perr2), perr2);

            assert_eq!(pok1.or_any(pok2), pok1);
            assert_eq!(pok1.or_any(ppart2), pok1);
            assert_eq!(pok1.or_any(perr2), pok1);

            assert_eq!(pok1.or_else_any(|_| pok2), pok1);
            assert_eq!(pok1.or_else_any(|_| ppart2), pok1);
            assert_eq!(pok1.or_else_any(|_| perr2), pok1);

            // PPart
            assert_eq!(ppart1.and_any(pok2), pok2);
            assert_eq!(ppart1.and_any(ppart2), ppart2);
            assert_eq!(ppart1.and_any(perr2), perr2);

            assert_eq!(ppart1.and_then_any(|_| pok2), pok2);
            assert_eq!(ppart1.and_then_any(|_| ppart2), ppart2);
            assert_eq!(ppart1.and_then_any(|_| perr2), perr2);

            assert_eq!(ppart1.or_any(pok2), pok2);
            assert_eq!(ppart1.or_any(ppart2), ppart2);
            assert_eq!(ppart1.or_any(perr2), perr2);

            assert_eq!(ppart1.or_else_any(|_| pok2), pok2);
            assert_eq!(ppart1.or_else_any(|_| ppart2), ppart2);
            assert_eq!(ppart1.or_else_any(|_| perr2), perr2);

            // PErr
            assert_eq!(perr1.and_any(pok2), perr1);
            assert_eq!(perr1.and_any(ppart2), perr1);
            assert_eq!(perr1.and_any(perr2), perr1);

            assert_eq!(perr1.and_then_any(|_| pok2), perr1);
            assert_eq!(perr1.and_then_any(|_| ppart2), perr1);
            assert_eq!(perr1.and_then_any(|_| perr2), perr1);

            assert_eq!(perr1.or_any(pok2), pok2);
            assert_eq!(perr1.or_any(ppart2), ppart2);
            assert_eq!(perr1.or_any(perr2), perr2);

            assert_eq!(perr1.or_else_any(|_| pok2), pok2);
            assert_eq!(perr1.or_else_any(|_| ppart2), ppart2);
            assert_eq!(perr1.or_else_any(|_| perr2), perr2);
        }

        #[test]
        fn unwrap() {
            let ppart = PPART;
            let (pok, perr) = (ppart.partial_into_pok(), ppart.partial_into_perr());

            assert_eq!(pok.unwrap_or(-123), 123);
            assert_eq!(pok.unwrap_or_else(|e| e), 123);
            assert_eq!(pok.unwrap_any_or(-123), 123);
            assert_eq!(pok.unwrap_any_or_else(|e| e), 123);

            assert_eq!(ppart.unwrap_or(-123), -123);
            assert_eq!(ppart.unwrap_or_else(|e| e), -123);
            assert_eq!(ppart.unwrap_any_or(-123), 123);
            assert_eq!(ppart.unwrap_any_or_else(|e| e), 123);

            assert_eq!(perr.unwrap_or(-123), -123);
            assert_eq!(perr.unwrap_or_else(|e| e), -123);
            assert_eq!(perr.unwrap_any_or(-123), -123);
            assert_eq!(perr.unwrap_any_or_else(|e| e), -123);
        }
    }
}
