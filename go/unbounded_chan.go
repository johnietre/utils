package utils

import (
	"container/list"
	"errors"
	"sync"
	"sync/atomic"
	"time"
)

var (
	// ErrClosed means an object is closed.
	ErrClosed = errors.New("closed")
	// ErrTimedOut means an operation timed out.
	ErrTimedOut = errors.New("timed out")
	// ErrCanceled means an operation was canceled.
	ErrCanceled = errors.New("canceled")
)

// UChan is an unbounded channel.
type UChan[T any] struct {
	ch       chan T
	buf      *list.List
	bufMtx   sync.Mutex
	isClosed atomic.Bool
}

// NewUChan returns a new UChan with the given chan length, `l`. `l` can
// realistically be any number, but having a higher number means it will be
// faster at the cost of more space.
func NewUChan[T any](l int) *UChan[T] {
	return &UChan[T]{
		ch:  make(chan T, l),
		buf: list.New(),
	}
}

// Recv receives from the channel, returning false if the channel is closed.
func (uc *UChan[T]) Recv() (T, bool) {
	t, ok := <-uc.ch
	if ok {
		uc.moveMsg()
	}
	return t, ok
}

// RecvTimeout receives from the channel within the given duration. If there is
// a value immediately available in the channel, the timeout is not used.
// Returns ErrClosed if the channel is closed and ErrTimedOut if the timeout is
// reached.
func (uc *UChan[T]) RecvTimeout(dur time.Duration) (t T, err error) {
	ok := false
RecvTimeoutLoop:
	for {
		select {
		case t, ok = <-uc.ch:
			if !ok {
				return t, ErrClosed
			}
			break RecvTimeoutLoop
		default:
		}
		timer := time.NewTimer(dur)
		select {
		case t, ok = <-uc.ch:
			timer.Stop()
			if !ok {
				return t, ErrClosed
			}
			break RecvTimeoutLoop
		case <-timer.C:
			return t, ErrTimedOut
		}
	}
	uc.moveMsg()
	return
}

// RecvCancel functions the same as RecvTimeout except takes a chan used to
// cancel the operation (if no value was immediately available). Sending over
// the cancel chan as well as closing it will cancel the operation, returning
// ErrCanceled.
func (uc *UChan[T]) RecvCancel(cancel chan struct{}) (t T, err error) {
	ok := false
RecvCancelLoop:
	for {
		select {
		case t, ok = <-uc.ch:
			if !ok {
				return t, ErrClosed
			}
			break RecvCancelLoop
		default:
		}
		select {
		case t, ok = <-uc.ch:
			if !ok {
				return t, ErrClosed
			}
			break RecvCancelLoop
		case _, _ = <-cancel:
			return t, ErrCanceled
		}
	}
	uc.moveMsg()
	return
}

// Receiver is returned by UChan.RecvChan to receive a value.
type Receiver[T any] struct {
	ch       chan T
	cancel   chan struct{}
	canceled atomic.Bool
}

// Chan returns the chan used to receive on.
func (r *Receiver[T]) Chan() <-chan T {
	return r.ch
}

// Cancel is used to cancel the receiver, returning whether the call canceled
// the receiver. Closes the Receiver's chan (from Receiver.Chan). Calling after
// a value has been sent to the Receiver does nothing (since Cancel is called
// after the value is sent).
func (r *Receiver[T]) Cancel() bool {
	if r.canceled.Swap(true) {
		return false
	}
	close(r.ch)
	close(r.cancel)
	return true
}

// RecvChan returns a Receiver used to receive a single value from the UChan.
// It's useful, for example, when selecting from the UChan and another chan.
// The chan in the receiver (from Reciever.Chan) should be drained no matter
// what. When done attempting to receive from the Receiver, Receiver.Cancel
// should be called (unless a value is actually received from the Receiver).
// The Receivers chan is closed in all circumstances. If a value is sent to the
// Receiver's chan, the chan is closed after the value is sent. If the UChan is
// closed, the Receiver's chan is closed without a value being sent.
func (uc *UChan[T]) RecvChan() *Receiver[T] {
	cancel := make(chan struct{})
	r := &Receiver[T]{
		ch:     make(chan T, 1),
		cancel: cancel,
	}
	go func() {
		t, err := uc.RecvCancel(cancel)
		if err == nil {
			r.ch <- t
		}
		r.Cancel()
	}()
	return r
}

func (uc *UChan[T]) moveMsg() {
	uc.bufMtx.Lock()
	defer uc.bufMtx.Unlock()
	if uc.buf.Len() == 0 {
		return
	}
	e := uc.buf.Front()
	uc.ch <- e.Value.(T)
	uc.buf.Remove(e)
	// If there are no more messages in the buffer and the UChan is closed, it's
	// safe to close the chan
	if uc.buf.Len() == 0 && uc.IsClosed() {
		close(uc.ch)
	}
}

// Send sends the value over the channel. This will never block until the
// channel is received from, though it may be slower if many calls to Send are
// made (due to locking).
func (uc *UChan[T]) Send(val T) bool {
	if uc.IsClosed() {
		return false
	}
	uc.bufMtx.Lock()
	defer uc.bufMtx.Unlock()
	for e := uc.buf.Front(); e != nil; e = e.Next() {
		select {
		case uc.ch <- e.Value.(T):
			tmp := e
			e = e.Next()
			uc.buf.Remove(tmp)
		default:
			uc.buf.PushBack(val)
			return true
		}
	}
	select {
	case uc.ch <- val:
	default:
		uc.buf.PushBack(val)
	}
	return true
}

// Close closes the channel, returning false if the channel was already closed.
func (uc *UChan[T]) Close() bool {
	if uc.isClosed.Swap(true) {
		return false
	}
	uc.bufMtx.Lock()
	defer uc.bufMtx.Unlock()
	// Nothing more will be sent over the channel; it's safe to close
	if uc.buf.Len() == 0 {
		close(uc.ch)
	}
	return true
}

// IsClosed returns whether the channel is closed.
func (uc *UChan[T]) IsClosed() bool {
	return uc.isClosed.Load()
}
