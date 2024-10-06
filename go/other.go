package utils

import (
	"encoding/json"
	"errors"
	"os"
	"time"
)

// Must panics if the error is not nil, otherwise, returns the value.
func Must[T any](t T, err error) T {
	if err != nil {
		panic(err)
	}
	return t
}

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
// value if `ptr` is nil.
func ValOrDefault[T any](ptr *T) (t T) {
	return ValOr(ptr, t)
}

// Or returns the first value that is not equal to the default value, returning
// the default value if there is no value matching this criteria.
func Or[T comparable](vals ...T) T {
	var t T
	for _, val := range vals {
		if val != t {
			return val
		}
	}
	return t
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

// MapPtr maps the value in a pointer to another type, returning a pointer to
// the new value. If the passed pointer is nil, the function `f` is not called.
func MapPtr[T, U any](t *T, f func(*T) *U) *U {
	if t == nil {
		return nil
	}
	return f(t)
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

// First discards the second value and returns the first.
func First[T any, U any](t T, u U) T {
	return t
}

// Second discards the first value and returns the second.
func Second[T any, U any](t T, u U) U {
	return u
}

// EnvFileOrVar attempts to read a value from a file specified by an
// environment variable. The name of the environment variable for the file is
// gotten by adding "_FILE" to the end of the passed envName. If the read is
// successful, the resulting value is trimmed of its spaces. If there is an
// error reading the file, the value associated with the passed envName is
// returned along with the error from attempting to read the file.
func EnvFileOrVar(envName string) (string, error) {
	envFileName := envName + "_FILE"
	filename := os.Getenv(envFileName)
	bytes, err := os.ReadFile(filename)
	val := strings.TrimSpace(string(bytes))
	if err != nil {
		val = os.Getenv(envName)
	}
	return val, err
}
