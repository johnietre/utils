package utils

import (
	"testing"
	"time"
)

func TestUChanBasic(t *testing.T) {
	ch := NewUChan[int](10)
	for i := 0; i < 100; i++ {
		if !ch.Send(i) {
			t.Fatal("channel unexpectedly closed")
		}
	}
	for i := 0; i < 100; i++ {
		if n, err := ch.RecvTimeout(time.Millisecond); err != nil {
			t.Fatal("unexpected error: ", err)
		} else if n != i {
			t.Fatalf("expected %d, got %d", i, n)
		}
	}
	if n, err := ch.RecvTimeout(time.Millisecond); err != ErrTimedOut {
		t.Fatalf("unexpected value or error: %d, %v", n, err)
	}

	if ch.IsClosed() {
		t.Fatal("channel unexpectedly closed")
	}
	if !ch.Send(-1) {
		t.Fatal("channel unexpectedly closed")
	}
	if !ch.Close() {
		t.Fatal("channel unexpectedly closed")
	}
	if n, err := ch.RecvTimeout(time.Millisecond); err != nil {
		t.Fatal("unexpected error:", err)
	} else if n != -1 {
		t.Fatalf("expected %d, got %d", -1, n)
	}

	if ch.Send(-2) {
		t.Fatal("channel not closed")
	}
	if ch.SendAndClose(-3) {
		t.Fatal("channel not closed")
	}
	if n, err := ch.RecvTimeout(time.Millisecond); err != ErrClosed {
		t.Fatalf("unexpected value or error: %d, %v", n, err)
	}
	if ch.Close() {
		t.Fatal("channel not closed")
	}
	if !ch.IsClosed() {
		t.Fatal("channel not closed")
	}
}

func TestUChanRecvChan(t *testing.T) {
	ch := NewUChan[int](10)
	done := make(chan bool, 10)
	for i := 0; i < 100; i++ {
		go func() {
			rch := ch.RecvChan()
			_, ok := <-rch.Chan()
			if !ok {
				t.Fatal("channel closed")
			}
			done <- true
		}()
	}
	for i := 0; i < 100; i++ {
		ch.Send(i)
	}

	timer := time.NewTimer(time.Second * 3)
	for i := 0; i < 100; i++ {
		select {
		case <-done:
		case <-timer.C:
			t.Fatal("timed out")
		}
	}
	if !timer.Stop() {
		<-timer.C
	}

	rch := ch.RecvChan()
	go func() {
		time.Sleep(time.Second * 1)
		// NOTE: Also (partially) tests UChan.RecvCancel
		rch.Cancel()
	}()
	timer.Reset(time.Second * 3)
	select {
	case n, ok := <-rch.Chan():
		if ok {
			t.Fatal("received unexpected value: ", n)
		}
	case <-timer.C:
		t.Fatal("timed out")
	}
	if !timer.Stop() {
		<-timer.C
	}

	for i := 0; i < 10; i++ {
		go func() {
			rch := ch.RecvChan()
			_, ok := <-rch.Chan()
			done <- ok
		}()
	}
	ch.SendAndClose(100)
	timer.Reset(time.Second * 3)
	valueGot := false
	for i := 0; i < 10; i++ {
		select {
		case got := <-done:
			if got {
				if valueGot {
					t.Fatal("value received more than once")
				}
				valueGot = true
			}
		case <-timer.C:
			t.Fatal("timed out")
		}
	}
	if !timer.Stop() {
		<-timer.C
	}
}
