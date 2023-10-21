package utils

import (
	"io"
	"sync"
)

// LockedWriter is a wrapper to lock writes on an underlying writer.
type LockedWriter struct {
	w   io.Writer
	mtx sync.Mutex
}

// NewLockedWriter returns a new LockedWriter.
func NewLockedWriter(w io.Writer) *LockedWriter {
	return &LockedWriter{w: w}
}

// Write locks (and unlocks) the writer and writes to the underlying writer.
func (lw *LockedWriter) Write(p []byte) (n int, err error) {
	lw.Lock()
	n, err = lw.LockedWrite(p)
	lw.Unlock()
	return
}

// LockedWrite writes to the underlying writer without locking. Useful if the
// lock is already held.
func (lw *LockedWriter) LockedWrite(p []byte) (n int, err error) {
	return lw.w.Write(p)
}

// TryWrite attempts to lock the writer and write to the underlying writer.
// Returns 0, nil, false if it failed to lock, otherwise, returns true along
// with the results of the write.
func (lw *LockedWriter) TryWrite(p []byte) (n int, err error, locked bool) {
	if locked = lw.TryLock(); !locked {
		return
	}
	n, err = lw.LockedWrite(p)
	lw.Unlock()
	return
}

// WriteAll locks (and unlocks) the writer and attempts to write all of the
// bytes passed. Returns err == nil iff n == len(p).
func (lw *LockedWriter) WriteAll(p []byte) (n int64, err error) {
	lw.Lock()
	n, err = lw.LockedWriteAll(p)
	lw.Unlock()
	return
}

// LockedWriteAll attempts to write all of the bytes passed without locking.
// Returns err == nil iff n == len(p).
func (lw *LockedWriter) LockedWriteAll(p []byte) (n int64, err error) {
	return WriteAll(lw.w, p)
}

// TryWriteAll attempts to lock (and subsequencly unlock) the writer and write
// all of the bytes passed. Returns err == nil iff n == len(p). Returns false
// if locking failed.
func (lw *LockedWriter) TryWriteAll(
	p []byte,
) (n int64, err error, locked bool) {
	if locked = lw.TryLock(); !locked {
		return
	}
	n, err = lw.LockedWriteAll(p)
	lw.Unlock()
	return
}

// LockWriter locks the writer and returns the underlying writer.
func (lw *LockedWriter) LockWriter() io.Writer {
	lw.Lock()
	return lw.w
}

// TryLockWriter attempts to lock the writer, returning false if it failed to
// lock.
func (lw *LockedWriter) TryLockWriter() (io.Writer, bool) {
	if !lw.TryLock() {
		return nil, false
	}
	return lw.w, true
}

// Lock locks the writer.
func (lw *LockedWriter) Lock() {
	lw.mtx.Lock()
}

// TryLock attempts to lock the writer, returning true if successful.
func (lw *LockedWriter) TryLock() bool {
	return lw.mtx.TryLock()
}

// Unlock unlocks the writer.
func (lw *LockedWriter) Unlock() {
	lw.mtx.Unlock()
}

// WriteAll attempts writes to write all bytes to the given writer. Returns
// err == nil iff n == len(p).
func WriteAll(w io.Writer, p []byte) (n int64, err error) {
	for nw, l := 0, int64(len(p)); n < l && err == nil; {
		nw, err = w.Write(p[n:])
		n += int64(nw)
	}
	return
}
