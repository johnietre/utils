package utils

import (
	"io"
)

// DeferFunc is meant to be used in `defer` statements. The passed function(s)
// will be run if `*shouldRun` is true at the time the function is executed
// (which is usually at the end of a function due to using `defer`).
// `shouldRun` should not be nil, otherwise, it will panic.
func DeferFunc(shouldRun *bool, funcs ...func()) {
	if shouldRun != nil && *shouldRun {
		for _, f := range funcs {
			f()
		}
	}
}

// DeferClose works the same as DeferFunc but works with io.Closers. Errors
// from called Close methods are ignored. If this is not desired, handle them
// with new funcs using `DeferFunc`.
func DeferClose(shouldRun *bool, closers ...io.Closer) {
	if shouldRun != nil && *shouldRun {
		for _, c := range closers {
			c.Close()
		}
	}
}

// DeferredCloser is meant to be be used to store closers that are do be closed
// at the time defers are ran. When this is run, the semantics are the same as
// `DeferClose`.
type DeferredCloser struct {
	closers   []io.Closer
	shouldRun func() bool
	ran       bool
}

// NewDeferredCloser returns a new DeferredCloser. By default, this does not
// run if it `Run` has already been called before. This can be changed by
// calling `SetShouldRun` or `SetShouldRunFunc`.
func NewDeferredCloser(shouldRun *bool) *DeferredCloser {
	dc := &DeferredCloser{
		closers: make([]io.Closer, 0),
		ran:     false,
	}
	dc.shouldRun = func() bool {
		return !dc.ran && shouldRun != nil && *shouldRun
	}
	return dc
}

// SetShouldRun changes the pointer used when checking if it should run. If not
// already done, this overrides it so that there is no check as to whether it
// has already run or not.
func (dc *DeferredCloser) SetShouldRun(shouldRun *bool) {
	dc.shouldRun = func() bool {
		return shouldRun != nil && *shouldRun
	}
}

// SetShouldRunFunc changes the func used when checking if it should run. If
// not already done, this overrides it so that there is no check as to whether
// it has already run or not. The check can still be done in the passed
// function, however.
func (dc *DeferredCloser) SetShouldRunFunc(shouldRun func() bool) {
	dc.shouldRun = shouldRun
}

// Add addes new closers to be closed.
func (dc *DeferredCloser) Add(closers ...io.Closer) {
	dc.closers = append(dc.closers, closers...)
}

// Ran returns whether this has run or not (`Run` has been called).
func (dc *DeferredCloser) Ran() bool {
	return dc.ran
}

// Run attempts to run this. This is usually called from a `defer` statement.
// Returns false if it did not run (i.e., if the shouldRun set was false).
func (dc *DeferredCloser) Run() bool {
	if !dc.shouldRun() {
		return false
	}
	for _, c := range dc.closers {
		c.Close()
	}
	dc.ran = true
	return true
}

// DeferredFunc is meant to be be used to store funcs that are do be run at the
// time defers are ran. When this is run, the semantics are the same as
// `DeferFunc`.
type DeferredFunc struct {
	funcs     []func()
	shouldRun func() bool
	ran       bool
}

// NewDeferredFunc returns a new DeferredFunc. By default, this does not
// run if it `Run` has already been called before. This can be changed by
// calling `SetShouldRun` or `SetShouldRunFunc`.
func NewDeferredFunc(shouldRun *bool) *DeferredFunc {
	dc := &DeferredFunc{
		funcs: make([]func(), 0),
		ran:   false,
	}
	dc.shouldRun = func() bool {
		return !dc.ran && shouldRun != nil && *shouldRun
	}
	return dc
}

// SetShouldRun changes the pointer used when checking if it should run. If not
// already done, this overrides it so that there is no check as to whether it
// has already run or not.
func (dc *DeferredFunc) SetShouldRun(shouldRun *bool) {
	dc.shouldRun = func() bool {
		return shouldRun != nil && *shouldRun
	}
}

// SetShouldRunFunc changes the func used when checking if it should run. If
// not already done, this overrides it so that there is no check as to whether
// it has already run or not. The check can still be done in the passed
// function, however.
func (dc *DeferredFunc) SetShouldRunFunc(shouldRun func() bool) {
	dc.shouldRun = shouldRun
}

// Add addes new funcs to be run.
func (dc *DeferredFunc) Add(funcs ...func()) {
	dc.funcs = append(dc.funcs, funcs...)
}

// Ran returns whether this has run or not (`Run` has been called).
func (dc *DeferredFunc) Ran() bool {
	return dc.ran
}

// Run attempts to run this. This is usually called from a `defer` statement.
// Returns false if it did not run (i.e., if the shouldRun set was false).
func (dc *DeferredFunc) Run() bool {
	if !dc.shouldRun() {
		return false
	}
	for _, f := range dc.funcs {
		f()
	}
	dc.ran = true
	return true
}
