package utils

import (
	"errors"
	"testing"
)

type testErr struct{}

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
