package utils

import (
	"errors"
	"testing"
)

type testErr Unit

func (*testErr) Error() string { return "" }

func TestErrAs(t *testing.T) {
	var err error
	err = &testErr{}
	if !ErrAs[*testErr](err) {
		t.Error("expected true")
	}

	err = errors.New("test err")
	if ErrAs[*testErr](err) {
		t.Error("expected false")
	}
}

func TestValOr(t *testing.T) {
	var val, other int = 123, 456
	if got := ValOr(&val, other); got != val {
		t.Errorf("expected %d, got %d", val, got)
	}
	if got := ValOr(nil, other); got != other {
		t.Errorf("expected %d, got %d", other, got)
	}

	newVal := func() int {
		return 789
	}
	if got := ValOrFunc(&val, newVal); got != val {
		t.Errorf("expected %d, got %d", val, got)
	}
	if got := ValOrFunc(nil, newVal); got != newVal() {
		t.Errorf("expected %d, got %d", newVal(), got)
	}

	var def int
	if got := ValOrDefault(&val); got != val {
		t.Errorf("expected %d, got %d", val, got)
	}
	if got := ValOrDefault[int](nil); got != def {
		t.Errorf("expected %d, got %d", def, got)
	}
}

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
