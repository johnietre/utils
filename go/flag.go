package utils

import "strings"

// BoolMapFlag is a map that holds whether various values were passed. This is
// intended to be used in cases such as passing a flag multiple times with
// different values to indicate failing on different types of errors (something
// similar to common flags like -W).
type BoolMapFlag map[string]bool

// NewBoolMapFlag creates a new BoolMapFlag.
func NewBoolMapFlag() BoolMapFlag {
	return make(BoolMapFlag)
}

// Has gets whether the given value was passed.
func (bm BoolMapFlag) Has(s string) bool {
	return bm[s]
}

// Unset unsets (deletes) the given value from the map. Only accepts one value.
func (bm BoolMapFlag) Unset(s string) {
	delete(bm, s)
}

// String implements the flag.Value interface, returning a string
// representation.
func (bm BoolMapFlag) String() string {
	res := ""
	for s := range bm {
		res += s + ","
	}
	if res != "" {
		res = res[:len(res)-1]
	}
	return res
}

// Set implements the flag.Value interface, adding the passed value to the map.
// This accepts a comma-separated list of values as well.
func (bm BoolMapFlag) Set(s string) error {
	parts := strings.Split(s, ",")
	for _, part := range parts {
		bm[part] = true
	}
	return nil
}
