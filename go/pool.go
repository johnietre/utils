package utils

import "sync"

// Pool is a non-synchronous resource pool.
// TODO: Not implemented
type Pool[T any] struct {
	newFunc any
	p       sync.Pool
}

func NewPool[T any](f func() (T, bool)) *Pool[T] {
	return &Pool[T]{
		p: sync.Pool{
			New: func() any {
				t, ok := f()
				if !ok {
					return nil
				}
				return t
			},
		},
	}
}

func AlwaysNewPool[T any](f func() T) *Pool[T] {
	return &Pool[T]{
		p: sync.Pool{
			New: func() any {
				return f()
			},
		},
	}
}

func (p *Pool[T]) Get() (t T) {
	i := p.p.Get()
	if i != nil {
		t = i.(T)
	}
	return
}

func (p *Pool[T]) GetOk() (t T, ok bool) {
	i := p.p.Get()
	if i != nil {
		t, ok = i.(T), true
	}
	return
}

func (p *Pool[T]) Put(t T) {
	p.p.Put(t)
}

func (p *Pool[T]) NewFunc() (func() (T, bool), bool) {
	f, ok := p.newFunc.(func() (T, bool))
	return f, ok
}

func (p *Pool[T]) AlwaysNewFunc() (func() T, bool) {
	f, ok := p.newFunc.(func() T)
	return f, ok
}

func (p *Pool[T]) IsAlwaysNew() bool {
	_, ok := p.newFunc.(func() T)
	return ok
}

// SyncPool is a typed sync.Pool.
type SyncPool[T any] struct {
	newFunc any
	p       sync.Pool
}

// NewSyncPool creates a new SyncPool.
func NewSyncPool[T any](f func() (T, bool)) *SyncPool[T] {
	newFunc := func() any {
		t, ok := f()
		if !ok {
			return nil
		}
		return t
	}
	if f == nil {
		f = func() (_ T, _ bool) {
			return
		}
		newFunc = func() any {
			return nil
		}
	}
	return &SyncPool[T]{
		newFunc: f,
		p: sync.Pool{
			New: newFunc,
		},
	}
}

// AlwaysNewSyncPool creates a SyncPool that can will always return a value
// when `Get` is called.
func AlwaysNewSyncPool[T any](f func() T) *SyncPool[T] {
	if f == nil {
		f = func() (t T) {
			return t
		}
	}
	return &SyncPool[T]{
		newFunc: f,
		p: sync.Pool{
			New: func() any {
				return f()
			},
		},
	}
}

// Get returns a value from the pool, or the default is none was returned.
func (p *SyncPool[T]) Get() (t T) {
	i := p.p.Get()
	if i != nil {
		t = i.(T)
	}
	return
}

// GetOk functions the same as GetOk but also returns false when no value was
// returned.
func (p *SyncPool[T]) GetOk() (t T, ok bool) {
	i := p.p.Get()
	if i != nil {
		t, ok = i.(T), true
	}
	return
}

// GetAny calls and returns the result of calling New on the underlying
// sync.Pool.
func (p *SyncPool[T]) GetAny() any {
	return p.p.New()
}

// Put puts a value into the pool.
func (p *SyncPool[T]) Put(t T) {
	p.p.Put(t)
}

// NewFunc returns the function used to create new values if not created using
// `AlwaysNewSyncPool`.
func (p *SyncPool[T]) NewFunc() (func() (T, bool), bool) {
	f, ok := p.newFunc.(func() (T, bool))
	return f, ok
}

// AlwaysNewFunc returns the function passed to `AlwaysNewSyncPool`, or
// `nil, false` if not created using that function.
func (p *SyncPool[T]) AlwaysNewFunc() (func() T, bool) {
	f, ok := p.newFunc.(func() T)
	return f, ok
}

// AsNewFunc returns the function used to create the SyncPool as NewFunc
// function (signature `func() (T, bool)`), regardless of whether it was
// created as such. Will always be non-nil.
func (p *SyncPool[T]) AsNewFunc() func() (T, bool) {
	f, ok := p.newFunc.(func() (T, bool))
	if !ok {
		f2, ok := p.newFunc.(func() T)
		if !ok {
			f = func() (_ T, _ bool) {
				return
			}
		} else {
			f = func() (T, bool) {
				return f2(), true
			}
		}
	}
	return f
}

// NewFuncAny returns the New function of the underlying sync.Pool.
func (p *SyncPool[T]) NewFuncAny() func() any {
	return p.p.New
}

// IsAlwaysNew returns whether the pool was created using `AlwaysNewSyncPool`.
func (p *SyncPool[T]) IsAlwaysNew() bool {
	_, ok := p.newFunc.(func() T)
	return ok
}
