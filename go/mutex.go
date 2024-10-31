package utils

import (
	"encoding/json"
	"reflect"
	"sync"
)

// Locker represents an object that can be locked, attempted to be locked, and
// unlocked.
type Locker[T any] interface {
	// Lock locks the object and returns a pointer to the data it holds.
	Lock() *T
	// TryLock attempts to lock the object, returning the pointer and true if
	// successful, or nil, false otherwise.
	TryLock() (*T, bool)
	// Unlock unlocks the object.
	Unlock()
	// Apply locks the object and passes the pointer to its data to the given
	// function.
	Apply(func(*T))
	// TryApply is the same as Apply but only makes an attempt to lock the
	// object, returning false is locking failed.
	TryApply(func(*T)) bool
}

// RLocker represents an object that can be read locked, attempted to be read
// locked, and read unlocked. The data returned from the R methods should
// not be mutated in order to withhold the usefulness of read locking.
type RLocker[T any] interface {
	Locker[T]
	// RLock read locks the object and returns a pointer to the data it holds.
	RLock() *T
	// TryRLock attempts to read lock the object, returning the pointer and true
	// if successful, or nil, false otherwise.
	TryRLock() (*T, bool)
	// RUnlock read unlocks the object.
	RUnlock()
	// RApply read locks the object and passes the pointer to its data to the
	// given function.
	RApply(func(*T))
	// TryRApply is the same as RApply but only makes an attempt to read lock the
	// object, returning false is read locking failed.
	TryRApply(func(*T)) bool
}

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
	defer m.Unlock()
	f(m.Lock())
}

// TryApply attempts to lock the mutex and call the passed function with a
// pointer to the data, returning true if successful.
func (m *Mutex[T]) TryApply(f func(*T)) bool {
	data, locked := m.TryLock()
	if locked {
		defer m.Unlock()
		f(data)
	}
	return locked
}

func (m *Mutex[T]) MarshalJSON() ([]byte, error) {
	m.Lock()
	defer m.Unlock()
	return json.Marshal(m.data)
}

func (m *Mutex[T]) UnmarshalJSON(data []byte) (err error) {
	m.Lock()
	defer m.Unlock()
	typ := reflect.TypeOf((*T)(nil)).Elem()
	if kind := typ.Kind(); kind == reflect.Pointer {
		val := reflect.ValueOf(m.data)
		if val.IsNil() {
			val = reflect.New(typ.Elem())
		}
		err = json.Unmarshal(data, val.Interface())
		m.data = val.Interface().(T)
		return
	} else if kind == reflect.Interface {
		return json.Unmarshal(data, reflect.ValueOf(m.data).Interface())
	}
	valPtr := reflect.ValueOf(&m.data)
	err = json.Unmarshal(data, valPtr.Interface())
	return
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
	defer m.Unlock()
	f(m.Lock())
}

// TryApply attempts to lock the mutex and call the passed function with a
// pointer to the data, returning true if successful.
func (m *RWMutex[T]) TryApply(f func(*T)) bool {
	data, locked := m.TryLock()
	if locked {
		defer m.Unlock()
		f(data)
	}
	return locked
}

// RApply read locks the mutex and calls the passed function with a pointer to
// the data. The data should not be mutated.
func (m *RWMutex[T]) RApply(f func(*T)) {
	defer m.Unlock()
	f(m.Lock())
}

// TryRApply attempts to read lock the mutex and call the passed function with
// a pointer to the data, returning true if successful. The data should not be
// mutated.
func (m *RWMutex[T]) TryRApply(f func(*T)) bool {
	data, locked := m.TryLock()
	if locked {
		defer m.Unlock()
		f(data)
	}
	return locked
}

func (m *RWMutex[T]) MarshalJSON() ([]byte, error) {
	m.RLock()
	defer m.RUnlock()
	return json.Marshal(m.data)
}

func (m *RWMutex[T]) UnmarshalJSON(data []byte) (err error) {
	m.Lock()
	defer m.Unlock()
	typ := reflect.TypeOf((*T)(nil)).Elem()
	if kind := typ.Kind(); kind == reflect.Pointer {
		val := reflect.ValueOf(m.data)
		if val.IsNil() {
			val = reflect.New(typ.Elem())
		}
		err = json.Unmarshal(data, val.Interface())
		m.data = val.Interface().(T)
		return
	} else if kind == reflect.Interface {
		return json.Unmarshal(data, reflect.ValueOf(m.data).Interface())
	}
	valPtr := reflect.ValueOf(&m.data)
	err = json.Unmarshal(data, valPtr.Interface())
	return
}
