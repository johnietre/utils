package utils

import (
	"sync/atomic"
)

// Unit is just an alias empty struct.
type Unit struct{}

// SyncSet is an alias for SyncMap[T, Unit]
type SyncSet[T any] struct {
	m    *SyncMap[T, Unit]
	size int64
}

// NewSyncSet returns a new SyncSet.
func NewSyncSet[T any]() *SyncSet[T] {
	return &SyncSet[T]{m: NewSyncMap[T, Unit]()}
}

// Insert inserts a value, returning true if the value didn't exist
func (s *SyncSet[T]) Insert(item T) bool {
	_, loaded := s.m.LoadOrStore(item, Unit{})
	if loaded {
		return false
	}
	atomic.AddInt64(&s.size, 1)
	return true
}

// Remove deletes a value, returning true if the value existed
func (s *SyncSet[T]) Remove(item T) bool {
	_, loaded := s.m.LoadAndDelete(item)
	if loaded {
		atomic.AddInt64(&s.size, -1)
		return true
	}
	return false
}

// Contains returns whether the set contains the item
func (s *SyncSet[T]) Contains(item T) bool {
	_, loaded := s.m.Load(item)
	return loaded
}

// Range iterates over each item in random order, applying a given function
// that returns whether the iterations should stop
func (s *SyncSet[T]) Range(f func(T) bool) {
	s.m.Range(func(k T, _ Unit) bool {
		return f(k)
	})
}

// SizeHint returns a hint at the possible number of entries in the set.
func (s *SyncSet[T]) SizeHint() int {
	return int(atomic.LoadInt64(&s.size))
}
