package utils

import (
	"errors"
	"sync/atomic"
)

// ErrorValue is used as a wrapper for the error interface that serves as a
// concrete type to satisfy atomic.Value when storing different errors.
type ErrorValue struct {
	// Error is the internal error.
	Error error
}

// NewErrorValue constructs a new error value with the given error.
func NewErrorValue(err error) ErrorValue {
	return ErrorValue{Error: err}
}

// Is implements errors.Is.
func (ev ErrorValue) Is(target error) bool {
	return errors.Is(ev.Error, target)
}

// As implements errors.As.
func (ev ErrorValue) As(target any) bool {
	return errors.As(ev.Error, target)
}

// Unwrap implements errors.Unwrap.
func (ev ErrorValue) Unwrap() error {
	return ev.Error
}

// AValue is an atomic value with a type using generics. Interfaces should
// generally not be passed as the generic since storing interfaces with
// different concrete types will result in a runtime panic. If an interface is
// needed, wrappers should be used which can wrap it in a concrete type (e.g.,
// ErrorValue for storing errors).
type AValue[T any] struct {
	v atomic.Value
}

// NewAValue constructs a new AValue with the given value. If no initial value
// is desired, create using struct literal (&AValue{}).
func NewAValue[T any](t T) *AValue[T] {
	var v atomic.Value
	v.Store(t)
	return &AValue[T]{v: v}
}

// Load loads the value. A value needs to be stored otherwise will panic.
func (a *AValue[T]) Load() T {
	return a.v.Load().(T)
}

// LoadSafe loads the value, returning the value and true. False and the
// default value are returned if there was no value stored.
func (a *AValue[T]) LoadSafe() (t T, ok bool) {
	iT := a.v.Load()
	if iT != nil {
		t = iT.(T)
	}
	return
}

// Store stores a value.
func (a *AValue[T]) Store(t T) {
	a.v.Store(t)
}

// Swap swaps the value, returning the old value. If there was no old value,
// false is returned.
func (a *AValue[T]) Swap(t T) (old T, ok bool) {
	oldV := a.v.Swap(t)
	if oldV == nil {
		return
	}
	return oldV.(T), true
}

// CompareAndSwap compares the provided old value with the value currently
// stored, swapping if they are equal. Returns true if swapped.
func (a *AValue[T]) CompareAndSwap(oldV, newV T) bool {
	return a.v.CompareAndSwap(oldV, newV)
}

// SwapIfEmpty stores the value if no value has been stored yet. Returns
// true if stored.
func (a *AValue[T]) StoreIfEmpty(t T) bool {
	return a.v.CompareAndSwap(nil, t)
}
