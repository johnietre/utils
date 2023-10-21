package utils

import "sync"

// SyncMap is a typed sync.Map.
type SyncMap[K any, V any] struct {
	m sync.Map
}

// NewSyncMap returns a new SyncMap.
func NewSyncMap[K any, V any]() *SyncMap[K, V] {
	return &SyncMap[K, V]{}
}

// Load loads the value for the given key.
func (m *SyncMap[K, V]) Load(key K) (value V, ok bool) {
	var v any
	if v, ok = m.m.Load(key); ok {
		value = v.(V)
	}
	return
}

// Store stores the given key/value pair.
func (m *SyncMap[K, V]) Store(key K, value V) {
	m.m.Store(key, value)
}

// LoadOrStore loads the value for the given key, or stores the given value if
// not present.
func (m *SyncMap[K, V]) LoadOrStore(
	key K, value V,
) (actual V, loaded bool) {
	var v any
	if v, loaded = m.m.LoadOrStore(key, value); loaded {
		actual = v.(V)
	} else {
		actual = value
	}
	return
}

// LoadAndDelete loads and deletes the given key, returning the value if there.
func (m *SyncMap[K, V]) LoadAndDelete(key K) (value V, loaded bool) {
	var v any
	if v, loaded = m.m.LoadAndDelete(key); loaded {
		value = v.(V)
	}
	return
}

// Delete deletes the key from the map.
func (m *SyncMap[K, V]) Delete(key K) {
	m.m.Delete(key)
}

// Range iterators through the list, passing the key/value pairs to f. If f
// returns false, iteration stops.
func (m *SyncMap[K, V]) Range(f func(key K, value V) bool) {
	m.m.Range(func(k, v any) bool {
		return f(k.(K), v.(V))
	})
}
