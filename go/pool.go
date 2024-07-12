package utils

import "sync"

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
