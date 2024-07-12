package utils

import (
	"testing"
)

type testCloser struct {
	closed bool
}

func (tc *testCloser) Close() error {
	tc.closed = true
	return nil
}

func TestDefer(t *testing.T) {
	// DeferFunc

	willDeferFunc := NewT(false)
	defer DeferFunc(
		willDeferFunc,
		func() {
			if !*willDeferFunc {
				t.Error("willDeferFunc 1 didn't run")
			}
		},
		func() {
			if !*willDeferFunc {
				t.Error("willDeferFunc 2 didn't run")
			}
		},
	)
	*willDeferFunc = true

	wontDeferFunc := NewT(true)
	defer DeferFunc(
		wontDeferFunc,
		func() {
			if *wontDeferFunc {
				t.Error("wontDeferFunc 1 ran")
			}
		},
		func() {
			if *wontDeferFunc {
				t.Error("wontDeferFunc 2 didn't ran")
			}
		},
	)
	*wontDeferFunc = false

	// DeferClose

	willClose, wontClose := &testCloser{}, &testCloser{}
	defer func() {
		if !willClose.closed {
			t.Error("willClose not closed")
		}
		if wontClose.closed {
			t.Error("wontClose closed")
		}
	}()

	willDeferClose, wontDeferClose := NewT(true), NewT(false)
	defer DeferClose(willDeferClose, willClose)
	defer DeferClose(wontDeferClose, wontClose)
}
