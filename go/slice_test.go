package utils

import (
	"math/rand"
	"testing"
	"time"
)

func generateSlice(l int, shuffle bool) []int {
	s := make([]int, l)
	for i := 0; i < l; i++ {
		s[i] = i
	}
	if shuffle {
		rand.Seed(time.Now().Unix())
		rand.Shuffle(l, func(i, j int) { s[i], s[j] = s[j], s[i] })
	}
	return s
}

func TestFilterSlice(t *testing.T) {
	s := generateSlice(1000, false)
	f := func(i int) bool {
		return i%2 == 1
	}
	want := make([]int, 500)
	for i := 1; i < 1000; i += 2 {
		want[i/2] = i
	}

	t.Run("FilterSlice", func(t *testing.T) {
		got := FilterSlice(s, f)
		if i := SliceCompare(got, want); i != -1 {
			if l := len(got); l != 500 {
				t.Errorf("want length of 500, got %d", l)
			} else {
				t.Errorf("index %d: want %d, got %d", i, got[i], want[i])
			}
		}
	})

	t.Run("FilterSliceInPlace", func(t *testing.T) {
		got := FilterSliceInPlace(CloneSlice(s), f)
		if i := SliceCompare(got, want); i != -1 {
			if l := len(got); l != 500 {
				t.Errorf("want length of 500, got %d", l)
			} else {
				t.Errorf("index %d: want %d, got %d", i, got[i], want[i])
			}
		}
	})

	t.Run("FilterSliceInPlaceUnstable", func(t *testing.T) {
		nums := make(map[int]bool, 500)
		for i := 1; i < 1000; i += 2 {
			nums[i] = true
		}
		got := FilterSliceInPlace(CloneSlice(s), f)
		for i, num := range got {
			if !nums[num] {
				t.Errorf("index %d: unexpected element %d", i, num)
			} else {
				nums[num] = false
			}
		}
		for num, b := range nums {
			if b {
				t.Errorf("missing element %d", num)
			}
		}
	})
}

func TestFilterMapSlice(t *testing.T) {
	s := generateSlice(1000, true)
	f := func(i int) (int, bool) {
		if i%2 == 1 {
			return 0, true
		}
		return -1, false
	}
	want := make([]int, 500)

	t.Run("FilterMapSlice", func(t *testing.T) {
		got := FilterMapSlice(s, f)
		if i := SliceCompare(got, want); i != -1 {
			if l := len(got); l != 500 {
				t.Errorf("want length of 500, got %d", l)
			} else {
				t.Errorf("index %d: want %d, got %d", i, got[i], want[i])
			}
		}
	})

	count := 0
	f = func(i int) (int, bool) {
		if i%2 == 1 {
			i = count
			count++
			return i, true
		}
		return -1, false
	}
	for i := range want {
		want[i] = i
	}
	t.Run("FilterMapSliceInPlace", func(t *testing.T) {
		got := FilterMapSlice(s, f)
		if i := SliceCompare(got, want); i != -1 {
			if l := len(got); l != 500 {
				t.Errorf("want length of 500, got %d", l)
			} else {
				t.Errorf("index %d: want %d, got %d", i, got[i], want[i])
			}
		}
	})
}

func TestSlice(t *testing.T) {
	const l = 1000
	rs := generateSlice(l, true)
	s := NewSlice(rs)

	t.Run("SliceIndexing", func(t *testing.T) {
		if s.Len() != l {
			t.Fatalf("expected length of %d, got %d", l, s.Len())
		}
		for i := 0; i < l; i++ {
			if s.Data()[i] != rs[i] {
				t.Fatalf("%d: expected %d, got %d", i, s.Data()[i], rs[i])
			} else if s.Data()[i] != s.Get(i) {
				t.Fatalf(
					"%d: Data()[i] != s.Get(i) (expected %d, got %d)",
					i, s.Data()[i], s.Get(i),
				)
			} else if s.GetPtr(i) != &rs[i] {
				t.Fatalf("%d: s.GetPtr(i) != &rs[i]", i)
			}
		}

		sliced := NewSlice(s.GetSlice(-1, -1))
		if sliced.Len() != l {
			t.Fatalf("expected length of %d, got %d", l, sliced.Len())
		}
		sliced = NewSlice(s.GetSlice(0, -1))
		if sliced.Len() != l {
			t.Fatalf("expected length of %d, got %d", l, sliced.Len())
		}

		start, end := l/3, (l/3)*2
		sliced, want := NewSlice(s.GetSlice(start, end)), rs[start:end]
		if sliced.Len() != len(want) {
			t.Fatalf("expected length of %d, got %d", len(want), sliced.Len())
		}
		for i := 0; i < sliced.Len(); i++ {
			if sliced.Get(i) != want[i] {
				t.Fatalf("%d: expected %d, got %d", i, want[i], sliced.Get(i))
			}
		}
	})

	t.Run("SafeSliceIndexing", func(t *testing.T) {
		i := l / 2
		if n, ok := s.GetSafe(i); !ok {
			t.Fatalf("%d: expected true, got false", i)
		} else if n != s.Get(i) {
			t.Fatalf("%d: expected %d, got %d", i, s.Get(i), n)
		}
		if ptr, ok := s.GetPtrSafe(i); !ok {
			t.Fatalf("%d: expected true, got false", i)
		} else if ptr != s.GetPtr(i) {
			t.Fatalf("%d: expected ptr %p, got ptr %p", i, s.GetPtr(i), ptr)
		} else if s.GetPtrNil(i) != s.GetPtr(i) {
			t.Fatalf(
				"%d: expected ptr %p, got ptr %p",
				i, s.GetPtr(i), s.GetPtrNil(i),
			)
		}

		start, end := l/3, (l/3)*2
		want := rs[start:end]
		slice, ok := s.GetSliceSafe(start, end)
		if !ok {
			t.Fatalf("expected true, got false")
		}
		sliced := NewSlice(slice)
		if sliced.Len() != len(want) {
			t.Fatalf("expected length of %d, got %d", len(want), sliced.Len())
		}
		for i := 0; i < sliced.Len(); i++ {
			if sliced.Get(i) != want[i] {
				t.Fatalf("%d: expected %d, got %d", i, want[i], sliced.Get(i))
			}
		}
		slice = s.GetSliceNil(start, end)
		if slice == nil {
			t.Fatalf("expected slice, got nil")
		}
		sliced = NewSlice(slice)
		if sliced.Len() != len(want) {
			t.Fatalf("expected length of %d, got %d", len(want), sliced.Len())
		}
		for i := 0; i < sliced.Len(); i++ {
			if sliced.Get(i) != want[i] {
				t.Fatalf("%d: expected %d, got %d", i, want[i], sliced.Get(i))
			}
		}

		// Functions shouuld fail
		if Second(s.GetSafe(l)) {
			t.Fatalf("%d: expected false, got true", l)
		}
		if ptr, ok := s.GetPtrSafe(l); ok {
			t.Fatalf("%d: expected false, got true", l)
		} else if ptr != nil {
			t.Fatalf("%d: expected nil, got ptr", l)
		}
		if s.GetPtrNil(l) != nil {
			t.Fatalf("%d: expected nil, got a ptr", l)
		}

		if Second(s.GetSliceSafe(-1, l+5)) {
			t.Fatal("expected false, got true")
		}
		if Second(s.GetSliceSafe(l+5, l+8)) {
			t.Fatal("expected false, got true")
		}
		if s.GetSliceNil(-1, l+5) != nil {
			t.Fatal("expected nil, got ptr")
		}
		if s.GetSliceNil(l+5, l+8) != nil {
			t.Fatal("expected nil, got ptr")
		}
	})

	// TODO: Rest of tests and check prior tests
}
