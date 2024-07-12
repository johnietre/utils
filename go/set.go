package utils

// Set is a wrapper for map[T]Unit.
type Set[T comparable] struct {
	m map[T]Unit
}

// NewSet creates a new set.
func NewSet[T comparable]() *Set[T] {
	return &Set[T]{m: make(map[T]Unit)}
}

// SetFromSlice creates a new set from the given slice. If duplicate values
// exist, the last value is kept.
func SetFromSlice[T comparable](s []T) *Set[T] {
	m := make(map[T]Unit, len(s))
	for _, v := range s {
		m[v] = Unit{}
	}
	return &Set[T]{m: m}
}

// Insert inserts a value, returning true if the value didn't exist.
func (s *Set[T]) Insert(item T) bool {
	if s.Contains(item) {
		return false
	}
	s.m[item] = Unit{}
	return true
}

// Remove deletes a value, returning true if the value existed.
func (s *Set[T]) Remove(item T) bool {
	if s.Contains(item) {
		delete(s.m, item)
		return true
	}
	return false
}

// Contains returns whether the set contains the item.
func (s *Set[T]) Contains(item T) bool {
	_, ok := s.m[item]
	return ok
}

// Range iterates over each item in random order, applying a given function
// that returns whether the iterations should stop.
func (s *Set[T]) Range(f func(T) bool) {
	for k := range s.m {
		if !f(k) {
			return
		}
	}
}

// Len returns the length of the set.
func (s *Set[T]) Len() int {
	return len(s.m)
}

// Clone clones the Set. If it is a set of pointers/interfaces, it does not
// attempt to clone the underlying values.
func (s *Set[T]) Clone() *Set[T] {
	return &Set[T]{m: CloneMap(s.m)}
}

// IntoMap copies the values of the set into the keys of the given map. Panics
// if `m` is nil.
func (s *Set[T]) IntoMap(m map[T]Unit) {
	CloneMapInto(m, s.m)
}

// ToMap returns a new map with the keys set to the values of the set.
func (s *Set[T]) ToMap() map[T]Unit {
	return CloneMap(s.m)
}
