package utils

import (
	"sync/atomic"
)

// Shared is a shared resource. Holders of this object should call the `Done`
// method once finished, so the object's return function can be called.
type Shared[T any] struct {
	val T
	num *atomic.Int64
	ret func(T)
}

// NewShared creates a new shared resource with the provided value and optional
// function to call when the value is no longer needed.
func NewShared[T any](val T, ret func(T)) Shared[T] {
	num := &atomic.Int64{}
	num.Add(1)
	return Shared[T]{
		val: val,
		num: num,
		ret: ret,
	}
}

// Val returns the stored value.
func (sb *Shared[T]) Val() T {
	return sb.val
}

// Clone clones the value. New instances that are to be passed elsewhere should
// be created using this method. Returns false if the this instance of the
// shared object has already be "freed" (`Done` has been called) or the object
// is invalid.
func (sb *Shared[T]) Clone() (Shared[T], bool) {
	if sb.num == nil {
		return Shared[T]{}, false
	}
	sb.num.Add(1)
	return Shared[T]{
		val: sb.val,
		num: sb.num,
		ret: sb.ret,
	}, true
}

// Done marks this instance of the shared object as finished with the resource.
// Returns true if this was the last to be done, in which case, the ret
// function is called, if one was set.
func (sb *Shared[T]) Done() bool {
	if sb.num == nil {
		return false
	}
	finished := false
	if sb.num.Add(-1) <= 0 {
		finished = true
		if sb.ret != nil {
			sb.ret(sb.val)
		}
	}
	sb.num = nil
	return finished
}
