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
