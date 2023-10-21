package utils

import (
	"strconv"
	"time"
)

// ParseFloat parses a float (alias for strconv.ParseFloat(f, 64)).
func ParseFloat(s string) float64 {
	f, _ := strconv.ParseFloat(s, 64)
	return f
}

// FormatFloat formats a float (alias for
// strconv.FormatFloat(f, 'f', -1, 64)).
func FormatFloat(f float64) string {
	return strconv.FormatFloat(f, 'f', -1, 64)
}

// ParseTime3339 parses a time from an RFC3339 string with nanoseconds (alias
// for time.Parse(time.RFC33339Nano, s)).
func ParseTime3339(s string) int64 {
	tt, _ := time.Parse(time.RFC3339Nano, s)
	// TODO: No if?
	if tt.IsZero() {
		return 0
	}
	return int64(tt.UnixNano())
}

// FormatTime3339 formats a timestamp with nanosecond precision to an RFC3339
// string with nanoseconds (alias for the appropriate time function(s)).
func FormatTime3339(u int64) string {
	t := int64(u)
	return time.Unix(t/1000000000, t%1000000000).Format(time.RFC3339Nano)
}
