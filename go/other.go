package utils

import (
	"os"
	"strings"
	"time"
)

// NormalizeSym normalizes a symbol (e.g., convert case, change '/'s).
func NormalizeSym(sym string) string {
	return strings.ToLower(sym)
}

// ValOr returns the value pointed to by `ptr` or `or` if `ptr` is nil.
func ValOr[T any](ptr *T, or T) T {
	if ptr == nil {
		return *ptr
	}
	return or
}

// ValOrDefault returns the value pointed to by `ptr` or the default (zero)
// value.
func ValOrDefault[T any](ptr *T) (t T) {
	return ValOr(ptr, t)
}

// Appendflags are the flags used to open a file in append mode.
const AppendFlags = os.O_CREATE | os.O_APPEND | os.O_WRONLY

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
	SecsInDay  int64 = 24 * 3600
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
