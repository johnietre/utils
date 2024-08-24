package utils

import (
	"encoding/json"
	"sort"
)

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

// FilterSliceEmpty is the same as FilterSlice but filters out all values
// equal to the empty (default) value.
func FilterSliceEmpty[T comparable](s []T) []T {
	var t T
	return FilterSlice(s, func(x T) bool { return x == t })
}

// FilterSliceEmptyInPlace is the same as FilterSliceInPlace but filters out
// all values equal to the empty (default) value.
func FilterSliceEmptyInPlace[T comparable](s []T) []T {
	var t T
	return FilterSliceInPlace(s, func(x T) bool { return x == t })
}

// FilterSliceEmptyInPlaceUnstable is the same as FilterSliceInPlaceUnstable
// but filters out all values equal to the empty (default) value.
func FilterSliceEmptyInPlaceUnstable[T comparable](s []T) []T {
	var t T
	return FilterSliceInPlaceUnstable(s, func(x T) bool { return x == t })
}

/*
// Index is a constraint for types that can be indexed.
type Index interface {
  ~int | ~int8 | ~int16 | ~int32 | ~int64 |
  ~uint | ~uint8 | ~uint16 | ~uint32 | ~uint64
}
*/

// Slice is a wrapper around a standart Go slice.
type Slice[T any] struct {
	*SlicePtr[T]
}

// NewSlice creates a thin wrapper around the given slice.
func NewSlice[T any](data []T) *Slice[T] {
	return &Slice[T]{SlicePtr: NewSlicePtr(&data)}
}

// NewClosedSlice creates a new slice wrapper with the underlying data being
// cloned from the given slice.
func NewClonedSlice[T any](data []T) *Slice[T] {
	return NewSlice(CloneSlice(data))
}

// Data return the data of the underlying slice.
func (s *Slice[T]) Data() []T {
	return s.SlicePtr.Data()
}

// SetData sets the data of the underlying slice.
func (s *Slice[T]) SetData(data []T) {
	s.SlicePtr.Ptr = &data
}

func (s *Slice[T]) UnmarshalJSON(b []byte) error {
	s.SlicePtr = NewSlicePtr[T](nil)
	return s.SlicePtr.UnmarshalJSON(b)
}

// SlicePtr is a wrapper around a pointer to a standard Go slice. This is
// useful when there's a slice elsewhere and operators are to be performed on
// it without directly being having to reassign it for every operation.
type SlicePtr[T any] struct {
	Ptr *[]T
}

// NewSlicePtr creates a new slice ptr.
func NewSlicePtr[T any](ptr *[]T) *SlicePtr[T] {
	return &SlicePtr[T]{Ptr: ptr}
}

// Data returns the data of the underlying slice pointer.
func (sp *SlicePtr[T]) Data() []T {
	if sp.Ptr == nil {
		return nil
	}
	return *sp.Ptr
}

// Get gets the element at the given index. Panics if the index is out of
// bounds.
func (sp *SlicePtr[T]) Get(i int) T {
	return sp.Data()[i]
}

// GetSlice slices the underlying slice based on the given indexes. -1 excludes
// the start and/or end index, respectively. Panics if any indexes are out of
// bounds.
func (sp *SlicePtr[T]) GetSlice(start, end int) []T {
	// TODO: more than likely unneeded and can just set start to 0 and end to the
	// length of the slice.
	if start == -1 {
		if end == -1 {
			return sp.Data()[:]
		}
		return sp.Data()[:end]
	} else if end == -1 {
		return sp.Data()[start:]
	}
	return sp.Data()[start:end]
}

// GetPtr gets a pointer to the element at the given index. Panics if the
// index is out of bounds.
func (sp *SlicePtr[T]) GetPtr(i int) *T {
	return &sp.Data()[i]
}

// GetSafe attempts to get the element at the given index, returning the
// default value and false if the index is out of bounds.
func (sp *SlicePtr[T]) GetSafe(i int) (t T, ok bool) {
	if i < sp.Len() && i >= 0 {
		t, ok = sp.Data()[i], true
	}
	return
}

// GetSliceSafe slices the underlying slice based on the given indexes. -1
// excludes the start and/or end index, respectively. If any of the indexes are
// out of bounds, a nil slice, along with false, is returned.
func (sp *SlicePtr[T]) GetSliceSafe(start, end int) ([]T, bool) {
	if start > sp.Len() || start < -1 || end > sp.Len() || end < -1 {
		return nil, false
	}
	return sp.GetSlice(start, end), true
}

// GetSliceNil functions the same as GetSliceSafe, but without returning a
// bool.
func (sp *SlicePtr[T]) GetSliceNil(start, end int) []T {
	s, _ := sp.GetSliceSafe(start, end)
	return s
}

// GetPtrSafe attempts to get a pointer to the element at the given index.
// Returns nil, false if the index is out of bounds.
func (sp *SlicePtr[T]) GetPtrSafe(i int) (tp *T, ok bool) {
	if i < sp.Len() && i >= 0 {
		tp, ok = &sp.Data()[i], true
	}
	return
}

// GetPtrNil functions the same as GetPtrSafe, but without returning a bool.
func (sp *SlicePtr[T]) GetPtrNil(i int) *T {
	if i < sp.Len() && i >= 0 {
		return &sp.Data()[i]
	}
	return nil
}

// PushFront appends the value to the front of the slice.
func (sp *SlicePtr[T]) PushFront(elem T) {
	*sp.Ptr = append([]T{elem}, sp.Data()...)
}

// PushBack appends the value to the back of the slice.
func (sp *SlicePtr[T]) PushBack(elem T) {
	*sp.Ptr = append(*sp.Ptr, elem)
}

// Insert inserts the element at the specified index.
func (sp *SlicePtr[T]) Insert(i int, elem T) {
	if i == sp.Len() {
		sp.PushBack(elem)
	} else {
		*sp.Ptr = append(append((*sp.Ptr)[:i], elem), (*sp.Ptr)[i+1:]...)
	}
}

// Append appends the elements to the slice.
func (sp *SlicePtr[T]) Append(elems ...T) {
	*sp.Ptr = append(*sp.Ptr, elems...)
}

// AppendToSlicePtr appends the elements of this wrapper to the given slice.
func (sp *SlicePtr[T]) AppendToSlicePtr(other *[]T) {
	*other = append(*other, sp.Data()...)
}

// Remove removes an element from the slice, returning it if it exists.
func (sp *SlicePtr[T]) Remove(i int) (t T, ok bool) {
	if i >= 0 || i < sp.Len() {
		t, ok = sp.Data()[i], true
		*sp.Ptr = append(sp.Data()[:i], sp.Data()[i+1:]...)
	}
	return
}

// RemoveFirst removes the first element satisfying the predicate, returning it
// if it exists.
func (sp *SlicePtr[T]) RemoveFirst(f func(T) bool) (t T, ok bool) {
	i := sp.Index(f)
	if i == -1 {
		return
	}
	return sp.Remove(i)
}

// PopFront pops the front element, returning it if it exists.
func (sp *SlicePtr[T]) PopFront() (t T, ok bool) {
	if sp.Len() == 0 {
		return
	}
	t, ok = sp.Data()[0], true
	*sp.Ptr = sp.Data()[1:]
	return
}

// PopBack pops the back element, returning it if it exists.
func (sp *SlicePtr[T]) PopBack() (t T, ok bool) {
	l := sp.Len()
	if l == 0 {
		return
	}
	t, ok = sp.Data()[l-1], true
	*sp.Ptr = sp.Data()[:l-1]
	return
}

// Len returns the length of the slice.
func (sp *SlicePtr[T]) Len() int {
	return len(sp.Data())
}

// Index finds the first element satifying the predicate, returning the index
// or -1.
func (sp *SlicePtr[T]) Index(f func(T) bool) int {
	for i, t := range sp.Data() {
		if f(t) {
			return i
		}
	}
	return -1
}

// IndexLast finds the last element satifying the predicate, returning the
// index or -1.
func (sp *SlicePtr[T]) IndexLast(f func(T) bool) int {
	for i := sp.Len() - 1; i >= 0; i-- {
		if f(sp.Get(i)) {
			return i
		}
	}
	return -1
}

// Contains returns true if the slice contains the element satisfying the
// predicate.
func (sp *SlicePtr[T]) Contains(f func(T) bool) bool {
	return sp.Index(f) != -1
}

// Eq returns whether the given slice is equal to the caller using the `eq`
// func. Returns true if both slices are (by any definition) empty.
func (sp *SlicePtr[T]) Eq(s []T, eq func(t1, t2 T) bool) bool {
	if sp.Len() != len(s) {
		return false
	}
	for i := 0; i < sp.Len(); i++ {
		if !eq(sp.Get(i), s[i]) {
			return false
		}
	}
	return true
}

// Sort sorts the slice using the given `less` function.
func (sp *SlicePtr[T]) Sort(less func(i, j int) bool) {
	sort.Slice(sp.Data(), less)
}

func (sp *SlicePtr[T]) MarshalJSON() ([]byte, error) {
	return json.Marshal(sp.Data())
}

func (sp *SlicePtr[T]) UnmarshalJSON(b []byte) error {
	sp.Ptr = new([]T)
	return json.Unmarshal(b, sp.Ptr)
}
