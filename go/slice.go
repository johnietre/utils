package utils

// CloneSlice clones a slice.
func CloneSlice[T any](s []T) []T {
	res := make([]T, len(s))
	copy(res, s)
	return res
}

// MapSlice maps a function onto a slice, returning a new slice. The returned
// slice has a capacity of the length of the given slice.
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

// FilterMapSlice filters and maps the elements of a slice, returning a new
// slice. The allocated slice has a capacity of the length of the given slice.
func FilterMapSlice[T, U any](s []T, f func(T) (U, bool)) []U {
	res := make([]U, 0, len(s))
	for _, t := range s {
		if u, ok := f(t); ok {
			res = append(res, u)
		}
	}
	return res
}

// FilterMapSliceInPlace filters and maps the elements of a slice with a given
// function, guaranteeing maintaining the original slice's order.
func FilterMapSliceInPlace[T any](s []T, f func(T) (T, bool)) []T {
	back := len(s) - 1
	for i := back; i >= 0; i-- {
		if t, ok := f(s[i]); !ok {
			if i != back {
				for j := i; j < back; j++ {
					s[j], s[j+1] = s[j+1], s[j]
				}
			}
			back--
		} else {
			s[i] = t
		}
	}
	return s[:back+1]
}

// FilterMapSliceInPlaceUnstable filters and maps the elements of a slice with
// a given function, without guaranteeing maintaining the original slice's
// order.
func FilterMapSliceInPlaceUnstable[T any](s []T, f func(T) (T, bool)) []T {
	back := len(s) - 1
	for i := back; i >= 0; i-- {
		if t, ok := f(s[i]); !ok {
			if i != back {
				s[i], s[back] = s[back], s[i]
			}
			back--
		} else {
			s[i] = t
		}
	}
	return s[:back+1]
}

// SearchSlice searches a slice for a given value, returning the index of -1.
func SearchSlice[T comparable](s []T, t T) int {
	for i, elem := range s {
		if elem == t {
			return i
		}
	}
	return -1
}

// SliceEq returns true if the slices are of equal length and all elements are
// equal. The returned slice has a capacity of the length of the given slice.
func SliceEq[T comparable](s1, s2 []T) bool {
	l := len(s1)
	if l != len(s2) {
		return false
	}
	for i := 0; i < l; i++ {
		if s1[i] != s2[i] {
			return false
		}
	}
	return true
}

// SliceCompare returns -1 if the slices are of equal length and all elements
// are equal, otherwise, returns the index of the first elements that aren't
// equal. If the slices are of differing lengths but one is a subslice of the
// other, the length of the subslice is returned.
func SliceCompare[T comparable](s1, s2 []T) int {
	l1, l2 := len(s1), len(s2)
	min, max := l1, l2
	if l1 > l2 {
		min, max = l2, l1
	} else if l1 < l2 {
		min, max = l1, l2
	}
	for i := 0; i < min; i++ {
		if s1[i] != s2[i] {
			return i
		}
	}
	if min != max {
		return min
	}
	return -1
}

// FilterSlice applies a predicate over each element in a slice, returning a
// new slice with the retained elements.
func FilterSlice[T any](s []T, f func(T) bool) []T {
	// TODO: Should it be initialized without a capacity
	res := make([]T, 0, len(s))
	for _, v := range s {
		if f(v) {
			res = append(res, v)
		}
	}
	return res
}

// FilterSliceInPlace filters a slice with a predicate returning a slice with
// the same underlying array, but with the elements kept by the filter placed
// in front. The length of the returned slice is the number of elements kept.
// The elements are guaranteed to stay in the same order and elemtns that
// weren't kept are placed at the end of the given slice.
func FilterSliceInPlace[T any](s []T, f func(T) bool) []T {
	back := len(s) - 1
	for i := back; i >= 0; i-- {
		if !f(s[i]) {
			if i != back {
				for j := i; j < back; j++ {
					s[j], s[j+1] = s[j+1], s[j]
				}
			}
			back--
		}
	}
	return s[:back+1]
}

// FilterSliceInPlaceUnstable filters a slice with a predicate returning a
// slice with the same underlying array, but with its elements potentially
// shuffled around and a length less than the original. Elements that are
// filtered out are placed at the end of the original slice (i.e., starting at
// the index of the length of the return).
func FilterSliceInPlaceUnstable[T any](s []T, f func(T) bool) []T {
	back := len(s) - 1
	for i := back; i >= 0; i-- {
		if !f(s[i]) {
			if i != back {
				s[i], s[back] = s[back], s[i]
			}
			back--
		}
	}
	return s[:back+1]

	/*
	  front, back := 0, len(s) - 1
	  for fr, b := front, back; fr >= b; fr, b = fr+1, b-1 {
	    if !f(s[fr]) {
	      if fr != front {
	        s[fr], s[front] = s[front], s[fr]
	      }
	      front++
	    }
	    if fr != b && !f(s[b]) {
	      if b != back {
	        s[b], s[back] = s[back], s[b]
	      }
	      back--
	    }
	  }
	  return s[front:back+1]
	*/
}
