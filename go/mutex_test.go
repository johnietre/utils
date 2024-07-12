package utils

import (
	"bytes"
	"encoding/json"
	"testing"
)

func TestMutexJSON(t *testing.T) {
	type testStruct struct {
		Field1 int    `json:"field1"`
		Field2 string `json:"field2"`
	}

	i := 123
	intMtx := NewMutex(i)
	b := Must(json.Marshal(i))
	if err := json.Unmarshal(b, intMtx); err != nil {
		t.Fatal("error unmarshaling value: ", err)
	}
	b2 := Must(json.Marshal(intMtx))
	if !bytes.Equal(b2, b) {
		t.Fatalf("bytes not equal: %v != %v", b2, b)
	}

	intMtx = &Mutex[int]{}
	b = Must(json.Marshal(i))
	if err := json.Unmarshal(b, intMtx); err != nil {
		t.Fatal("error unmarshaling value: ", err)
	}
	b2 = Must(json.Marshal(intMtx))
	if !bytes.Equal(b2, b) {
		t.Fatalf("bytes not equal: %v != %v", b2, b)
	}

	ts := testStruct{Field1: 1, Field2: "yes"}
	tsMtx := NewMutex(ts)
	tsMtx.Apply(func(tsp *testStruct) {
		tsp.Field1 = 12345
	})
	b = Must(json.Marshal(ts))
	if err := json.Unmarshal(b, tsMtx); err != nil {
		t.Fatal("error unmarshaling value: ", err)
	}
	b2 = Must(json.Marshal(tsMtx))
	if !bytes.Equal(b2, b) {
		t.Fatalf("bytes not equal: %v != %v", b2, b)
	}

	tsMtx = &Mutex[testStruct]{}
	b = Must(json.Marshal(ts))
	if err := json.Unmarshal(b, tsMtx); err != nil {
		t.Fatal("error unmarshaling value: ", err)
	}
	b2 = Must(json.Marshal(tsMtx))
	if !bytes.Equal(b2, b) {
		t.Fatalf("bytes not equal: %v != %v", b2, b)
	}

	tsp := &testStruct{Field1: 1, Field2: "yes"}
	tspMtx := NewMutex(tsp)
	b = Must(json.Marshal(tsp))
	tspMtx.Apply(func(ts **testStruct) {
		(*ts).Field1 = 12345
	})
	if err := json.Unmarshal(b, tspMtx); err != nil {
		t.Fatal("error unmarshaling value: ", err)
	}
	b2 = Must(json.Marshal(tspMtx))
	if !bytes.Equal(b2, b) {
		t.Fatalf("bytes not equal: %v != %v", b2, b)
	}

	tspMtx = &Mutex[*testStruct]{}
	b = Must(json.Marshal(tsp))
	if err := json.Unmarshal(b, tspMtx); err != nil {
		t.Fatal("error unmarshaling value: ", err)
	}
	b2 = Must(json.Marshal(tspMtx))
	if !bytes.Equal(b2, b) {
		t.Fatalf("bytes not equal: %v != %v", b2, b)
	}
}

func TestRWMutexJSON(t *testing.T) {
	type testStruct struct {
		Field1 int    `json:"field1"`
		Field2 string `json:"field2"`
	}

	i := 123
	intMtx := NewRWMutex(i)
	b := Must(json.Marshal(i))
	if err := json.Unmarshal(b, intMtx); err != nil {
		t.Fatal("error unmarshaling value: ", err)
	}
	b2 := Must(json.Marshal(intMtx))
	if !bytes.Equal(b2, b) {
		t.Fatalf("bytes not equal: %v != %v", b2, b)
	}

	intMtx = &RWMutex[int]{}
	b = Must(json.Marshal(i))
	if err := json.Unmarshal(b, intMtx); err != nil {
		t.Fatal("error unmarshaling value: ", err)
	}
	b2 = Must(json.Marshal(intMtx))
	if !bytes.Equal(b2, b) {
		t.Fatalf("bytes not equal: %v != %v", b2, b)
	}

	ts := testStruct{Field1: 1, Field2: "yes"}
	tsMtx := NewRWMutex(ts)
	tsMtx.Apply(func(tsp *testStruct) {
		tsp.Field1 = 12345
	})
	b = Must(json.Marshal(ts))
	if err := json.Unmarshal(b, tsMtx); err != nil {
		t.Fatal("error unmarshaling value: ", err)
	}
	b2 = Must(json.Marshal(tsMtx))
	if !bytes.Equal(b2, b) {
		t.Fatalf("bytes not equal: %v != %v", b2, b)
	}

	tsMtx = &RWMutex[testStruct]{}
	b = Must(json.Marshal(ts))
	if err := json.Unmarshal(b, tsMtx); err != nil {
		t.Fatal("error unmarshaling value: ", err)
	}
	b2 = Must(json.Marshal(tsMtx))
	if !bytes.Equal(b2, b) {
		t.Fatalf("bytes not equal: %v != %v", b2, b)
	}

	tsp := &testStruct{Field1: 1, Field2: "yes"}
	tspMtx := NewRWMutex(tsp)
	b = Must(json.Marshal(tsp))
	tspMtx.Apply(func(ts **testStruct) {
		(*ts).Field1 = 12345
	})
	if err := json.Unmarshal(b, tspMtx); err != nil {
		t.Fatal("error unmarshaling value: ", err)
	}
	b2 = Must(json.Marshal(tspMtx))
	if !bytes.Equal(b2, b) {
		t.Fatalf("bytes not equal: %v != %v", b2, b)
	}

	tspMtx = &RWMutex[*testStruct]{}
	b = Must(json.Marshal(tsp))
	if err := json.Unmarshal(b, tspMtx); err != nil {
		t.Fatal("error unmarshaling value: ", err)
	}
	b2 = Must(json.Marshal(tspMtx))
	if !bytes.Equal(b2, b) {
		t.Fatalf("bytes not equal: %v != %v", b2, b)
	}
}
