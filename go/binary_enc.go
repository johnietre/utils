package utils

import (
	"math"
	"unsafe"
)

func Put2(u uint16) []byte {
	return []byte{byte(u >> 8), byte(u)}
}

func Place2(b []byte, u uint16) {
	b[0] = byte(u >> 8)
	b[1] = byte(u)
}

func Get2(b []byte) uint16 {
	return uint16(b[0])<<8 | uint16(b[1])
}

func Put4(u uint32) []byte {
	return []byte{
		byte(u >> 24),
		byte(u >> 16),
		byte(u >> 8),
		byte(u),
	}
}

func Place4(b []byte, u uint32) {
	b[0] = byte(u >> 24)
	b[1] = byte(u >> 16)
	b[2] = byte(u >> 8)
	b[3] = byte(u)
}

func Get4(b []byte) uint32 {
	return uint32(b[0])<<24 |
		uint32(b[1])<<16 |
		uint32(b[2])<<8 |
		uint32(b[3])
}

func Put8(u uint64) []byte {
	return []byte{
		byte(u >> 56),
		byte(u >> 48),
		byte(u >> 40),
		byte(u >> 32),
		byte(u >> 24),
		byte(u >> 16),
		byte(u >> 8),
		byte(u),
	}
}

func Place8(b []byte, u uint64) {
	b[0] = byte(u >> 56)
	b[1] = byte(u >> 48)
	b[2] = byte(u >> 40)
	b[3] = byte(u >> 32)
	b[4] = byte(u >> 24)
	b[5] = byte(u >> 16)
	b[6] = byte(u >> 8)
	b[7] = byte(u)
}

func Get8(b []byte) uint64 {
	return uint64(b[0])<<56 |
		uint64(b[1])<<48 |
		uint64(b[2])<<40 |
		uint64(b[3])<<32 |
		uint64(b[4])<<24 |
		uint64(b[5])<<16 |
		uint64(b[6])<<8 |
		uint64(b[7])
}

func PutF(f float64) []byte {
	return Put8(math.Float64bits(f))
}

func PlaceF(b []byte, f float64) {
	Place8(b, math.Float64bits(f))
}

func GetF(b []byte) float64 {
	return math.Float64frombits(Get8(b))
}

type Unsigned interface {
	~uint16 | uint32 | uint64
}

func Put[T Unsigned](i int) []byte {
	u := T(i)
	size := unsafe.Sizeof(u)
	b := make([]byte, size)
	for i := size - 1; i >= 0; i-- {
		b[i] = byte(u)
		u >>= 8
	}
	return b
}

func Place[T Unsigned](b []byte, i int) {
	u := T(i)
	size := unsafe.Sizeof(u)
	for i := size - 1; i >= 0; i-- {
		b[i] = byte(u)
		u >>= 8
	}
}

func Get[T Unsigned](b []byte) T {
	var res T
	size := int(unsafe.Sizeof(res))
	for i := 0; i < size; i++ {
		res = (res << 8) | T(b[i])
	}
	return res
}
