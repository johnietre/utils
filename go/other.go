package utils

import (
	"encoding/json"
	"errors"
	"io"
	"os"
	"time"
)

// IsMarshalError returns whether the error is from calling Marshal or the
// process of marshaling. Useful in cases like json.Encoder.Encode where the
// error could be with the underlying writer.
func IsMarshalError(err error) bool {
	me := &json.MarshalerError{}
	ute, uve := &json.UnsupportedTypeError{}, &json.UnsupportedValueError{}
	return errors.As(err, &ute) || errors.As(err, &uve) || errors.As(err, &me)
}

// IsUnmarshalError returns whether the error is from the unmarshaling itself.
// This means that an InvalidUnmarshalError returns false since it's an error
// with the call itself, not the unmarshaling process.
func IsUnmarshalError(err error) bool {
	ute, se := &json.UnmarshalTypeError{}, &json.SyntaxError{}
	return errors.As(err, &ute) || errors.As(err, &se)
}

// ErrAs is a shorthand for the following:
// var et *ErrType
// errors.As(err, &et)
func ErrAs[T error](err error) bool {
	return errors.As(err, new(T))
}

// ValOr returns the value pointed to by `ptr` or `or` if `ptr` is nil.
func ValOr[T any](ptr *T, or T) T {
	if ptr != nil {
		return *ptr
	}
	return or
}

// ValOrFunc returns the value pointed to by `ptr` or the return value
// `orFunc`, which is only run if `ptr` is nil.
func ValOrFunc[T any](ptr *T, orFunc func() T) T {
	if ptr != nil {
		return *ptr
	}
	return orFunc()
}

// ValOrDefault returns the value pointed to by `ptr` or the default (zero)
// value.
func ValOrDefault[T any](ptr *T) (t T) {
	return ValOr(ptr, t)
}

// Appendflags are the flags used to open a file in append mode.
const AppendFlags = os.O_CREATE | os.O_APPEND | os.O_WRONLY

// OpenAppend is shorthand for calling os.OpenFile(path, AppendFlags, 0644).
func OpenAppend(path string) (*os.File, error) {
	return os.OpenFile(path, AppendFlags, 0644)
}

// NewT returns the pointer to a new T with the given value.
func NewT[T any](t T) *T {
	ptr := new(T)
	*ptr = t
	return ptr
}

// CurrentDay returns the current time with the hours, minutes, and seconds
// removed.
func CurrentDay() time.Time {
	t := time.Now()
	return time.Date(t.Year(), t.Month(), t.Day(), 0, 0, 0, 0, time.Local)
}

const (
	// SecsInDay is the seconds in a day
	SecsInDay int64 = 24 * 3600
	// NanosInDay is the nanoseconds in a day.
	NanosInDay int64 = int64(time.Hour) * 24
)

// TimestampToDay takes a timestamp with second-precision and returns a
// timestamp of the beginning of the day.
func TimestampToDay(i int64) int64 {
	return i - i%SecsInDay
}

// TimestampNanoToDay takes a timestamp with nanosecond-precision and returns a
// timestamp of the beginning of the day.
func TimestampNanoToDay(i int64) int64 {
	return i - i%NanosInDay
}

// DeferFunc is meant to be used in `defer` statements. The passed function(s)
// will be run if `*shouldRun` is true at the time the function is executed
// (which is usually at the end of a function due to using `defer`).
// `shouldRun` should not be nil, otherwise, it will panic.
func DeferFunc(shouldRun *bool, funcs ...func()) {
	if *shouldRun {
		for _, f := range funcs {
			f()
		}
	}
}

// DeferClose works the same as DeferFunc but works with io.Closers. Errors
// from called Close methods are ignored. If this is not desired, handle them
// with new funcs using `DeferFunc`.
func DeferClose(shouldRun *bool, closers ...io.Closer) {
	if *shouldRun {
		for _, c := range closers {
			c.Close()
		}
	}
}
