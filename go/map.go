package utils

// Map is a wrapper for a map[K]V
type Map[K comparable, V any] struct {
	m map[K]V
}

// NewMap creates a new Map.
func NewMap[K comparable, V any]() *Map[K, V] {
	return &Map[K, V]{m: make(map[K]V)}
}

// MapFromMap creates a new Map from the given Map.
func MapFromMap[K comparable, V any](m map[K]V) *Map[K, V] {
	if m == nil {
		m = make(map[K]V)
	}
	return &Map[K, V]{m: m}
}

// Set sets the key to the value.
func (m *Map[K, V]) Set(key K, value V) {
	m.m[key] = value
}

// Insert inserts (sets) the key and value, returning the old value it it
// existed.
func (m *Map[K, V]) Insert(key K, value V) (old V, inserted bool) {
	old, inserted = m.m[key]
	m.m[key] = value
	return
}

// TrySet inserts (sets) the key/value pair only if it does not already exist
// in the Map. Otherwise, false is returned.
func (m *Map[K, V]) TrySet(key K, value V) bool {
	if _, ok := m.m[key]; ok {
		return false
	}
	return true
}

// Get gets the value for the key or returns the default. This is the
// equivalent of `_ = (map[K]V)[key]`.
func (m *Map[K, V]) Get(key K) V {
	return m.m[key]
}

// GetOk gets the value for the key, returning true if it exists, or returns
// the default and false otherwise. This is the equivalent of
// `_, _ = (map[K]V)[key]`.
func (m *Map[K, V]) GetOk(key K) (V, bool) {
	v, ok := m.m[key]
	return v, ok
}

// GetByValue gets first key/value pair for which the value satisfies the given
// predicate, returning false if one doesn't exist. This is nondeterministic
// (since it uses Go maps).
func (m *Map[K, V]) GetByValue(pred func(V) bool) (k K, v V, ok bool) {
	for key, val := range m.m {
		if pred(val) {
			return key, val, true
		}
	}
	return
}

// Len returns the length of the Map.
func (m *Map[K, V]) Len() int {
	return len(m.m)
}

// ContainsKey returns whether the map contains the given key.
func (m *Map[K, V]) ContainsKey(key K) bool {
	_, ok := m.m[key]
	return ok
}

// ContainsValue returns whether the map contains a value that satisfies the
// given predicate function.
func (m *Map[K, V]) ContainsValue(pred func(V) bool) bool {
	for _, v := range m.m {
		if pred(v) {
			return true
		}
	}
	return false
}

// Delete deletes the value from the map for the given key.
func (m *Map[K, V]) Delete(key K) {
	delete(m.m, key)
}

// GetDelete gets the value then deletes it, if it exists, returning true if it
// existed.
func (m *Map[K, V]) GetDelete(key K) (V, bool) {
	val, ok := m.m[key]
	if ok {
		delete(m.m, key)
	}
	return val, ok
}

// Range iterates over each item in random order, applying a given function
// that returns whether the iterations should stop.
func (m *Map[K, V]) Range(f func(K, V) bool) {
	for k, v := range m.m {
		if !f(k, v) {
			return
		}
	}
}

// Clone clones the Map. If it is a set of pointers/interfaces, it does not
// attempt to clone the underlying values.
func (s *Map[K, V]) Clone() *Map[K, V] {
	return &Map[K, V]{m: CloneMap(s.m)}
}

// ToGoMap clones the inner map and returns it.
func (m *Map[K, V]) ToGoMap() map[K]V {
	return CloneMap(m.m)
}

// Inner returns the inner go map.
func (m *Map[K, V]) Inner() map[K]V {
	return m.m
}

// CloneMap clonse a map.
func CloneMap[K comparable, V any](m map[K]V) map[K]V {
	nm := make(map[K]V, len(m))
	for k, v := range m {
		nm[k] = v
	}
	return nm
}

// CloneMapInto copies the key/value pairs from `src` into `dst`, returning
// `dst` (not a new map). Panics if `dst` is nil.
func CloneMapInto[K comparable, V any](dst, src map[K]V) map[K]V {
	for k, v := range src {
		dst[k] = v
	}
	return dst
}
