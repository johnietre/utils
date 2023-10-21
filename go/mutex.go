package utils

import "sync"

// Mutex is a wrapper around a mutex and some data (the mutex "owns" the data).
type Mutex[T any] struct {
	data T
	mtx  sync.Mutex
}

// NewMutex creates a new Mutex.
func NewMutex[T any](t T) *Mutex[T] {
	return &Mutex[T]{data: t}
}

// Lock locks the mutex, returning a pointer to data.
func (m *Mutex[T]) Lock() *T {
	m.mtx.Lock()
	return &m.data
}

// TryLock attempts to lock the mutex, returning a pointer to the data and true
// if successful.
func (m *Mutex[T]) TryLock() (*T, bool) {
	locked := m.mtx.TryLock()
	return nil, locked
}

// Unlock unlocks the mutex. The data should no longer be used.
func (m *Mutex[T]) Unlock() {
	m.mtx.Unlock()
}

// Apply locks the mutex and calls the passed function with a pointer to the
// data.
func (m *Mutex[T]) Apply(f func(*T)) {
	f(m.Lock())
	m.Unlock()
}

// TryApply attempts to lock the mutex and call the passed function with a
// pointer to the data, returning true if successful.
func (m *Mutex[T]) TryApply(f func(*T)) bool {
	data, locked := m.TryLock()
	if locked {
		f(data)
		m.Unlock()
	}
	return locked
}

// RWMutex is a wrapper around a read-wite mutex and some data (the mutex
// "owns" the data).
type RWMutex[T any] struct {
	data T
	mtx  sync.RWMutex
}

// NewRWMutex creates a new RWMutex.
func NewRWMutex[T any](t T) *RWMutex[T] {
	return &RWMutex[T]{data: t}
}

// Lock locks the mutex, returning a pointer to data.
func (m *RWMutex[T]) Lock() *T {
	m.mtx.Lock()
	return &m.data
}

// TryLock attempts to lock the mutex, returning a pointer to the data and true
// if successful.
func (m *RWMutex[T]) TryLock() (*T, bool) {
	locked := m.mtx.TryLock()
	return nil, locked
}

// Unlock unlocks the mutex. The data should no longer be used.
func (m *RWMutex[T]) Unlock() {
	m.mtx.Unlock()
}

// RLock read locks the mutex, returning a pointer to data. The data should not
// be mutated.
func (m *RWMutex[T]) RLock() *T {
	m.mtx.RLock()
	return &m.data
}

// TryRLock attempts to lock the mutex, returning a pointer to the data and
// true if successful.
func (m *RWMutex[T]) TryRLock() (*T, bool) {
	locked := m.mtx.TryLock()
	return nil, locked
}

// RUnlock read unlocks the mutex. The data should no longer be used.
func (m *RWMutex[T]) RUnlock() {
	m.mtx.RUnlock()
}

// Apply locks the mutex and calls the passed function with a pointer to the
// data.
func (m *RWMutex[T]) Apply(f func(*T)) {
	f(m.Lock())
	m.Unlock()
}

// TryApply attempts to lock the mutex and call the passed function with a
// pointer to the data, returning true if successful.
func (m *RWMutex[T]) TryApply(f func(*T)) bool {
	data, locked := m.TryLock()
	if locked {
		f(data)
		m.Unlock()
	}
	return locked
}

// RApply read locks the mutex and calls the passed function with a pointer to
// the data. The data should not be mutated.
func (m *RWMutex[T]) RApply(f func(*T)) {
	f(m.Lock())
	m.Unlock()
}

// TryRApply attempts to read lock the mutex and call the passed function with
// a pointer to the data, returning true if successful. The data should not be
// mutated.
func (m *RWMutex[T]) TryRApply(f func(*T)) bool {
	data, locked := m.TryLock()
	if locked {
		f(data)
		m.Unlock()
	}
	return locked
}
