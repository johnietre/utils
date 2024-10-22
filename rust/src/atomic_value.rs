pub use std::sync::atomic::Ordering;
use std::sync::{atomic::AtomicPtr, Arc};

pub type AV<T> = AtomicValue<T>;

/// A container to safely share values atomically between threads.
/// The underlying value is dropped when the `AtomicValue` is dropped, if there was a value.
///
/// This type has the same in-memory representation as `AtomicPtr<T>`.
#[repr(transparent)]
pub struct AtomicValue<T>(AtomicPtr<T>);

impl<T: Sync + Clone> AtomicValue<T> {
    /// Creates a new `AtomicValue<T>` with the given value.
    /// It is (or at least should be) safe to call all methods on the return value of this
    /// function, barring any changes made through the internal `AtomicPtr`.
    #[inline(always)]
    pub fn new(val: T) -> Self {
        Self(AtomicPtr::new(Box::into_raw(Box::new(val))))
    }

    /// Creates a new `AtomicValue<T>` with a null pointer internally.
    #[inline(always)]
    pub const fn empty() -> Self {
        Self(AtomicPtr::new(0 as _))
    }

    /// Creates a new `AtomicValue<T>` with the given pointer to a value. This value should have
    /// been created with the global allocator since anytime a value needs to be dropped, it will
    /// be done using `Box::from_raw`.
    ///
    /// # Safety
    ///
    /// It is up to the caller to guarantee the pointer is a valid pointer to a T (or a null
    /// pointer).
    #[inline(always)]
    pub const unsafe fn from_val_ptr(ptr: *mut T) -> Self {
        Self(AtomicPtr::new(ptr))
    }

    /// Attempts to load and clone the value stored, returning None if no value has been stored.
    #[inline(always)]
    pub fn load(&self, order: Ordering) -> Option<T> {
        unsafe { self.0.load(order).as_ref().cloned() }
    }

    /// Loads and clones the value stored.
    ///
    /// # Safety
    ///
    /// Calling this method without a value having been stored results in a null pointer
    /// dereference.
    #[inline(always)]
    pub unsafe fn load_unchecked(&self, order: Ordering) -> T {
        (*self.0.load(order)).clone()
    }

    /// Stores a value, dropping the old one, if necessary.
    #[inline(always)]
    pub fn store(&self, val: T, order: Ordering) {
        let ptr = self.0.swap(Box::into_raw(Box::new(val)), order);
        if !ptr.is_null() {
            let _ = unsafe { Box::from_raw(ptr) };
        }
    }

    /// Stores a value, dropping the old one. `AtomicValue::store` should almost always be used
    /// rather than this method.
    ///
    /// # Safety
    ///
    /// Calling this method without a value having been stored results in an attempt to drop a null
    /// pointer.
    #[inline(always)]
    pub unsafe fn store_unchecked(&self, val: T, order: Ordering) {
        let _ = Box::from_raw(self.0.swap(Box::into_raw(Box::new(val)), order));
    }

    /// Stores a value if no value has yet been stored. Uses compare_exchange under the hood.
    ///
    /// NOTE: This will always Box the value and, on failure, "unbox" the value.
    #[inline(always)]
    pub fn store_if_empty(&self, val: T, success: Ordering, failure: Ordering) -> Result<(), T> {
        let ptr = Box::into_raw(Box::new(val));
        self.0
            .compare_exchange(0 as _, ptr, success, failure)
            .map(|_| ())
            .map_err(|_| unsafe { *Box::from_raw(ptr) })
    }

    /// Stores a value if no value has yet been stored. Uses compare_exchange_weak under the hood.
    ///
    /// NOTE: This will always Box the value and, on failure, "unbox" the value.
    #[inline(always)]
    pub fn store_if_empty_weak(
        &self,
        val: T,
        success: Ordering,
        failure: Ordering,
    ) -> Result<(), T> {
        let ptr = Box::into_raw(Box::new(val));
        self.0
            .compare_exchange_weak(0 as _, ptr, success, failure)
            .map(|_| ())
            .map_err(|_| unsafe { *Box::from_raw(ptr) })
    }

    /// Swaps the current value for the given value, returning the old one, if it exists.
    #[inline(always)]
    pub fn swap(&self, val: T, order: Ordering) -> Option<T> {
        let ptr = self.0.swap(Box::into_raw(Box::new(val)), order);
        if !ptr.is_null() {
            unsafe { Some(*Box::from_raw(ptr)) }
        } else {
            None
        }
    }

    /// Swaps the current value for the given value, returning the old one.
    ///
    /// # Safety
    ///
    /// Calling this method without a value having been stored results in a null pointer
    /// dereference.
    #[inline(always)]
    pub unsafe fn swap_unchecked(&self, val: T, order: Ordering) -> T {
        *Box::from_raw(self.0.swap(Box::into_raw(Box::new(val)), order))
    }

    /// Takes the value stored without replacing it, if it exists.
    #[inline(always)]
    pub fn take(&self, order: Ordering) -> Option<T> {
        let ptr = self.0.swap(0 as _, order);
        if !ptr.is_null() {
            unsafe { Some(*Box::from_raw(ptr)) }
        } else {
            None
        }
    }

    /// Takes the value stored without replacing it.
    ///
    /// # Safety
    ///
    /// Calling this method without a value having been stored results in a null pointer
    /// dereference.
    #[inline(always)]
    pub unsafe fn take_unchecked(&self, order: Ordering) -> T {
        *Box::from_raw(self.0.swap(0 as _, order))
    }

    /// Returns whether there is a value stored or not.
    #[inline(always)]
    pub fn is_empty(&self, order: Ordering) -> bool {
        self.0.load(order).is_null()
    }

    /// Gets the underlying `AtomicPtr<T>`.
    ///
    /// # Safety
    ///
    /// The `AtomicValue<T>` has no knowledge of the `AtomicPtr<T>`, and therefore, the owning
    /// `AtomicValue<T>` may be put in an invalid state, like if an invalid value is stored.
    #[inline(always)]
    pub unsafe fn as_atomic_ptr<'a>(&'a self) -> &'a AtomicPtr<T> {
        &self.0
    }
}

impl<T: Sync + Copy> AtomicValue<T> {
    /// Attempts to load and copy the value stored, returning None if no value has been stored.
    #[inline(always)]
    pub fn load_copied(&self, order: Ordering) -> Option<T> {
        unsafe { self.0.load(order).as_ref().copied() }
    }

    /// Loads and copies the value stored.
    ///
    /// # Safety
    ///
    /// Calling this method without a value having been stored results in a null pointer
    /// dereference.
    #[inline(always)]
    pub unsafe fn load_copied_unchecked(&self, order: Ordering) -> T {
        unsafe { *self.0.load(order) }
    }
}

impl<T: Sync + Clone + Default> AtomicValue<T> {
    /// Creates a new `AtomicValue<T>` with the default value of %.
    #[inline(always)]
    pub fn new_default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Sync + Clone> Default for AtomicValue<T> {
    /// Creates a new empty `AtomicValue<T>`.
    fn default() -> Self {
        Self::empty()
    }
}

impl<T> Drop for AtomicValue<T> {
    fn drop(&mut self) {
        let ptr = self.0.load(Ordering::SeqCst);
        if !ptr.is_null() {
            let _ = unsafe { Box::from_raw(ptr) };
        }
    }
}

pub type AAV<T> = AtomicArcValue<T>;

/// A wrapper, with accompanying methods, for `AtomicValue<Arc<T>>`.
///
/// This type has the same in-memory representation as `AtomicValue<T>` and follows the same rules.
#[repr(transparent)]
pub struct AtomicArcValue<T: ?Sized>(AtomicValue<Arc<T>>);

impl<T: Send + Sync> AtomicArcValue<T> {
    /// Creates a new `AtomicArcValue<T>` with the given value.
    #[inline(always)]
    pub fn new(val: T) -> Self {
        Self::from_arc(Arc::new(val))
    }

    /// Stores a value, dropping the old Arc, if necessary.
    #[inline(always)]
    pub fn store(&self, val: T, order: Ordering) {
        self.0.store(Arc::new(val), order);
    }

    /// Stores a value, dropping the old ARc. `AtomicValue::store` should almost always be used
    /// rather than this method.
    ///
    /// # Safety
    ///
    /// Calling this method without a value having been stored results in an attempt to drop a null
    /// pointer.
    #[inline(always)]
    pub unsafe fn store_unchecked(&self, val: T, order: Ordering) {
        self.0.store_unchecked(Arc::new(val), order);
    }

    /// Stores a value if no value has yet been stored. Uses compare_exchange under the hood.
    ///
    /// NOTE: This will always Box the value and, on failure, "unbox" the value.
    #[inline(always)]
    pub fn store_if_empty(&self, val: T, success: Ordering, failure: Ordering) -> Result<(), T> {
        self.0
            .store_if_empty(Arc::new(val), success, failure)
            .map_err(|v| Arc::into_inner(v).unwrap())
    }

    /// Stores a value if no value has yet been stored. Uses compare_exchange_weak under the hood.
    ///
    /// NOTE: This will always Box the value and, on failure, "unbox" the value.
    #[inline(always)]
    pub fn store_if_empty_weak(
        &self,
        val: T,
        success: Ordering,
        failure: Ordering,
    ) -> Result<(), T> {
        self.0
            .store_if_empty_weak(Arc::new(val), success, failure)
            .map_err(|v| Arc::into_inner(v).unwrap())
    }

    /// Swaps the current value for the given value, returning the old one, if it exists.
    #[inline(always)]
    pub fn swap(&self, val: T, order: Ordering) -> Option<Arc<T>> {
        self.0.swap(Arc::new(val), order)
    }

    /// Swaps the current value for the given value, returning the old one.
    ///
    /// # Safety
    ///
    /// Calling this method without a value having been stored results in a null pointer
    /// dereference.
    #[inline(always)]
    pub unsafe fn swap_unchecked(&self, val: T, order: Ordering) -> Arc<T> {
        self.0.swap_unchecked(Arc::new(val), order)
    }
}

impl<T: ?Sized + Send + Sync> AtomicArcValue<T> {
    /// Creates a new `AtomicArcValue<T>` with the given `Arc<T>`.
    #[inline(always)]
    pub fn from_arc(val: Arc<T>) -> Self {
        Self(AtomicValue::new(val))
    }

    /// Creates a new `AtomicValue<T>` with a null pointer internally.
    #[inline(always)]
    pub const fn empty() -> Self {
        Self(AtomicValue::empty())
    }

    /// Creates a new `AtomicValue<T>` with the given pointer to an `Arc<T>` . This value should
    /// have been created with the global allocator since anytime a value needs to be dropped, it
    /// will be done using `Box::from_raw`.
    ///
    /// # Safety
    ///
    /// It is up to the caller to guarantee the pointer is a valid pointer to an `Arc<T>`. (or a
    /// null pointer).
    #[inline(always)]
    pub const unsafe fn from_val_ptr(ptr: *mut Arc<T>) -> Self {
        Self(AtomicValue::from_val_ptr(ptr))
    }

    /// Attempts to load the value stored, returning None if no value has been stored.
    #[inline(always)]
    pub fn load(&self, order: Ordering) -> Option<Arc<T>> {
        unsafe { self.0 .0.load(order).as_ref().map(Arc::clone) }
    }

    /// Loads the value stored.
    ///
    /// # Safety
    ///
    /// Calling this method without a value having been stored results in a null pointer
    /// dereference.
    #[inline(always)]
    pub unsafe fn load_unchecked(&self, order: Ordering) -> Arc<T> {
        Arc::clone(&*self.0 .0.load(order))
    }

    /// Same as `AtomicArcValue::store` but takes an `Arc<T>`.
    #[inline(always)]
    pub fn store_arc(&self, val: Arc<T>, order: Ordering) {
        self.0.store(val, order)
    }

    /// Same as `AtomicArcValue::store_unchecked` but takes an `Arc<T>`.
    ///
    /// # Safety
    ///
    /// See `AtomicArcValue::store_unchecked`.
    #[inline(always)]
    pub unsafe fn store_arc_unchecked(&self, val: Arc<T>, order: Ordering) {
        self.0.store_unchecked(val, order)
    }

    /// Same as `AtomicArcValue::store_if_empty` but takes an `Arc<T>`.
    #[inline(always)]
    pub fn store_arc_if_empty(
        &self,
        val: Arc<T>,
        success: Ordering,
        failure: Ordering,
    ) -> Result<(), Arc<T>> {
        self.0.store_if_empty(val, success, failure)
    }

    /// Same as `AtomicArcValue::store_if_empty_weak` but takes an `Arc<T>`.
    #[inline(always)]
    pub fn store_arc_if_empty_weak(
        &self,
        val: Arc<T>,
        success: Ordering,
        failure: Ordering,
    ) -> Result<(), Arc<T>> {
        self.0.store_if_empty_weak(val, success, failure)
    }

    /// Same as `AtomicArcValue::swap` but takes an `Arc<T>`.
    #[inline(always)]
    pub fn swap_arc(&self, val: Arc<T>, order: Ordering) -> Option<Arc<T>> {
        self.0.swap(val, order)
    }

    /// Same as `AtomicArcValue::swap_unchecked` but takes an `Arc<T>`.
    ///
    /// # Safety
    ///
    /// See `AtomicArcValue::swap_unchecked`.
    #[inline(always)]
    pub unsafe fn swap_arc_unchecked(&self, val: Arc<T>, order: Ordering) -> Arc<T> {
        self.0.swap_unchecked(val, order)
    }

    /// Takes the value stored without replacing it, if it exists.
    #[inline(always)]
    pub fn take(&self, order: Ordering) -> Option<Arc<T>> {
        self.0.take(order)
    }

    /// Takes the value stored without replacing it.
    ///
    /// # Safety
    ///
    /// Calling this method without a value having been stored results in a null pointer
    /// dereference.
    #[inline(always)]
    pub unsafe fn take_unchecked(&self, order: Ordering) -> Arc<T> {
        self.0.take_unchecked(order)
    }

    /// Returns whether there is a value stored or not.
    #[inline(always)]
    pub fn is_empty(&self, order: Ordering) -> bool {
        self.0.is_empty(order)
    }

    /// Gets the underlying `AtomicPtr<Arc<T>>`.
    ///
    /// # Safety
    ///
    /// See `AtomicValue<T>::as_atomic_ptr`.
    #[inline(always)]
    pub unsafe fn as_atomic_ptr<'a>(&'a self) -> &'a AtomicPtr<Arc<T>> {
        self.0.as_atomic_ptr()
    }
}

impl<T: Send + Sync + Default> AtomicArcValue<T> {
    /// Creates a new `AtomicArcValue<T>` with the default value of %.
    #[inline(always)]
    pub fn new_default() -> Self {
        Self::from_arc(Arc::default())
    }
}

impl<T: Send + Sync> Default for AtomicArcValue<T> {
    /// Creates a new empty `AtomicArcValue<T>`.
    fn default() -> Self {
        Self::empty()
    }
}

pub type NEAV<T> = NEAtomicValue<T>;

/// The non-empty (NE) version of `AtomicValue<T>`, a container to safely share values atomically
/// between threads.
/// The value is (should) never empty, thus, all operations are done under that assumption and no
/// checks are done.
/// The underlying value is dropped when the `NEAtomicValue` is dropped.
///
/// This type has the same in-memory representation as `AtomicValue<T>`.
#[repr(transparent)]
pub struct NEAtomicValue<T>(AtomicValue<T>);

impl<T: Sync + Clone> NEAtomicValue<T> {
    /// Creates a new `NEAtomicValue<T>` with the given value.
    #[inline(always)]
    pub fn new(val: T) -> Self {
        Self(AtomicValue::new(val))
    }

    /// Creates a new `AtomicValue<T>` with the given pointer to a value. This value should have
    /// been created with the global allocator since anytime a value needs to be dropped, it will
    /// be done using `Box::from_raw`.
    ///
    /// # Safety
    ///
    /// It is up to the caller to guarantee the pointer is a non-null, valid pointer to a T.
    #[inline(always)]
    pub const unsafe fn from_val_ptr(ptr: *mut T) -> Self {
        Self(AtomicValue::from_val_ptr(ptr))
    }

    /// Loads and clones the value stored.
    #[inline(always)]
    pub fn load(&self, order: Ordering) -> T {
        unsafe { self.0.load_unchecked(order) }
    }

    /// Stores a value, dropping the old one.
    #[inline(always)]
    pub fn store(&self, val: T, order: Ordering) {
        unsafe {
            self.0.store_unchecked(val, order);
        }
    }

    /// Swaps the current value for the given value, returning the old one.
    #[inline(always)]
    pub fn swap(&self, val: T, order: Ordering) -> T {
        unsafe { self.0.swap_unchecked(val, order) }
    }

    /// Gets the underlying `AtomicPtr<T>`.
    ///
    /// # Safety
    ///
    /// See `AtomicValue::as_atomic_ptr`.
    #[inline(always)]
    pub unsafe fn as_atomic_ptr<'a>(&'a self) -> &'a AtomicPtr<T> {
        self.0.as_atomic_ptr()
    }
}

impl<T: Sync + Copy> NEAtomicValue<T> {
    /// Loads and copies the value stored, returning it.
    #[inline(always)]
    pub fn load_copied(&self, order: Ordering) -> T {
        unsafe { self.0.load_copied_unchecked(order) }
    }
}

impl<T: Sync + Clone + Default> Default for NEAtomicValue<T> {
    /// Returns a new `NEAtomicArcValue` with the value's default stored.
    fn default() -> Self {
        Self::new(T::default())
    }
}

pub type NEAAV<T> = NEAtomicArcValue<T>;

/// Non-empty (NE) version of `AtomicArcValue`. Follows the same rules as `NEAtomicValue`.
#[repr(transparent)]
pub struct NEAtomicArcValue<T: ?Sized>(AtomicArcValue<T>);

impl<T: Send + Sync> NEAtomicArcValue<T> {
    /// Creates a new `NEAtomicArcValue<T>` with the given value.
    #[inline(always)]
    pub fn new(val: T) -> Self {
        Self(AtomicArcValue::new(val))
    }

    /// Stores a value, dropping the old one.
    #[inline(always)]
    pub fn store(&self, val: T, order: Ordering) {
        unsafe {
            self.0.store_unchecked(val, order);
        }
    }

    /// Swaps the current value for the given value, returning the old one.
    #[inline(always)]
    pub fn swap(&self, val: T, order: Ordering) -> Arc<T> {
        unsafe { self.0.swap_unchecked(val, order) }
    }
}

impl<T: ?Sized + Send + Sync> NEAtomicArcValue<T> {
    /// Creates a new `NEAtomicArcValue<T>` with the given `Arc<T>`.
    #[inline(always)]
    pub fn from_arc(val: Arc<T>) -> Self {
        Self(AtomicArcValue::from_arc(val))
    }

    /// Creates a new `NEAtomicArcValue<T>` with the given pointer to an `Arc<T>`. This value
    /// should have been created with the global allocator since anytime a value needs to be
    /// dropped, it will be done using `Box::from_raw`.
    ///
    /// # Safety
    ///
    /// It is up to the caller to guarantee the pointer is a non-null, valid pointer to an
    /// `Arc<T>`.
    #[inline(always)]
    pub const unsafe fn from_val_ptr(ptr: *mut Arc<T>) -> Self {
        Self(AtomicArcValue::from_val_ptr(ptr))
    }

    /// Loads and clones the value stored.
    #[inline(always)]
    pub fn load(&self, order: Ordering) -> Arc<T> {
        unsafe { self.0.load_unchecked(order) }
    }

    /// Same as `NEAtomicArcValue::store` but takes an `Arc<T>`.
    #[inline(always)]
    pub fn store_arc(&self, val: Arc<T>, order: Ordering) {
        unsafe { self.0.store_arc_unchecked(val, order) }
    }

    /// Same as `NEAtomicArcValue::swap` but takes an `Arc<T>`.
    #[inline(always)]
    pub fn swap_arc(&self, val: Arc<T>, order: Ordering) -> Arc<T> {
        unsafe { self.0.swap_arc_unchecked(val, order) }
    }

    /// Gets the underlying `AtomicPtr<T>`.
    ///
    /// # Safety
    ///
    /// See `AtomicValue::as_atomic_ptr`.
    #[inline(always)]
    pub unsafe fn as_atomic_ptr<'a>(&'a self) -> &'a AtomicPtr<Arc<T>> {
        self.0.as_atomic_ptr()
    }
}

impl<T: Send + Sync + Default> Default for NEAtomicArcValue<T> {
    /// Returns a new `NEAtomicArcValue` with the value's default stored.
    fn default() -> Self {
        Self::new(T::default())
    }
}

#[cfg(test)]
mod test_single {
    use super::*;
    use std::sync::atomic::Ordering;

    #[test]
    fn empty_atomic_value() {
        let av = AtomicValue::empty();
        assert_eq!(av.load(Ordering::Relaxed), None);
        assert_eq!(av.swap(0, Ordering::Relaxed), None);
        assert_eq!(av.swap(1, Ordering::Relaxed), Some(0));

        let av = AtomicValue::default();
        assert_eq!(av.load(Ordering::Relaxed), None);
        assert_eq!(av.swap(0, Ordering::Relaxed), None);
        assert_eq!(av.swap(1, Ordering::Relaxed), Some(0));
    }

    #[test]
    fn atomic_value() {
        let av = AtomicValue::new(0);
        assert_eq!(av.load(Ordering::Relaxed), Some(0));
        assert_eq!(av.swap(1, Ordering::Relaxed), Some(0));
        av.store(15, Ordering::Relaxed);
        assert_eq!(av.swap(3, Ordering::Relaxed), Some(15));

        let av = AtomicValue::<i32>::new_default();
        assert_eq!(av.load(Ordering::Relaxed), Some(0));
        assert_eq!(av.swap(1, Ordering::Relaxed), Some(0));
        av.store(15, Ordering::Relaxed);
        assert_eq!(av.swap(3, Ordering::Relaxed), Some(15));
    }

    #[test]
    fn ne_atomic_arc_value() {
        let s1: Arc<str> = "abcde".into();
        let s2: Arc<str> = "fghij".into();
        let s3: Arc<str> = "klmno".into();
        let av = NEAtomicArcValue::from_arc(Arc::clone(&s1));
        assert_eq!(av.load(Ordering::Relaxed), s1);
        assert_eq!(av.swap_arc(Arc::clone(&s2), Ordering::Relaxed), s1);
        av.store_arc(Arc::clone(&s3), Ordering::Relaxed);
        assert_eq!(av.swap_arc("".into(), Ordering::Relaxed), s3);
    }

    #[test]
    fn store_if_empty() {
        let av = AtomicValue::empty();
        assert!(av
            .store_if_empty(123, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok());
        assert_eq!(
            av.store_if_empty(456, Ordering::Relaxed, Ordering::Relaxed),
            Err(456)
        );
        assert_eq!(av.load(Ordering::Relaxed), Some(123));

        assert!(!av.is_empty(Ordering::Relaxed));
        assert_eq!(av.take(Ordering::Relaxed), Some(123));
        assert!(av.is_empty(Ordering::Relaxed));
        assert_eq!(av.take(Ordering::Relaxed), None);
        assert!(av
            .store_if_empty(456, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok());
        assert_eq!(av.load(Ordering::Relaxed), Some(456));
    }
}

#[cfg(test)]
mod test_concurrent {}
