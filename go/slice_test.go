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
