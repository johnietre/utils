package utils

import (
  "container/list"
  "sync"
  "sync/atomic"
  "time"
)

type UChan[T any] struct {
  ch chan T
  buf *list.List
  bufMtx sync.Mutex
  isClosed atomic.Bool
}

func NewUChan[T any](l int) *UChan[T] {
  return &UChan[T]{
    ch: make(chan T, l),
    buf: list.New(),
  }
}

func (uc *UChan[T]) Recv() (T, bool) {
  t, ok := <-uc.ch
  if !ok {
    return t, ok
  }
  uc.moveMsg()
}

func (uc *UChan[T]) RecvTimeout(dur time.Duration) (t T, ) {
RecvTimeoutLoop:
  for {
    select {
    case t, ok <- uc.buf:
      if !ok {
        return // TODO
      }
      break RecvTimeoutLoop
    default:
    }
    timer := time.NewTimer(dur)
    select {
    case t, ok <- uc.buf:
      timer.Stop()
      if !ok {
        return // TODO
      }
      break RecvTimeoutLoop
    case <-timer.C:
      return // TODO
    }
  }
  s.moveMsg()
  return // TODO
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

func (uc *UChan[T]) Close() {
  if uc.isClosed.Swap(true) {
    return
  }
  uc.bufMtx.Lock()
  defer uc.bufMtx.Unlock()
  // Nothing more will be sent over the channel; it's safe to close
  if uc.buf.Len() == 0 {
    close(uc.ch)
  }
}

func (uc *UChan[T]) IsClosed() bool {
  return uc.isClosed.Load()
}
