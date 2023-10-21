package utils

// CloneSlice clones a slice.
func CloneSlice[T any](s []T) []T {
	res := make([]T, len(s))
	copy(res, s)
	return res
}

// MapSlice maps a function onto a slice, returning a new slice.
func MapSlice[T, U any](s []T, f func(T) U) []U {
	res := make([]U, 0, len(s))
	for _, v := range s {
		res = append(res, f(v))
	}
	return res
}

// MapSliceInPlace maps a function onto a slice, returning the same slice.
func MapSliceInPlace[T any](s []T, f func(T) T) []T {
	for i, v := range s {
		s[i] = f(v)
	}
	return s
}

// Search searches a slice for a given value, returning the index of -1.
func Search[T comparable](s []T, t T) int {
	for i, elem := range s {
		if elem == t {
			return i
		}
	}
	return -1
}
