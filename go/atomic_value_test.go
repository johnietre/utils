package utils

import (
	"bytes"
	"encoding/json"
	"testing"
)

func TestAValueJSON(t *testing.T) {
	type testStruct struct {
		Field1 int    `json:"field1"`
		Field2 string `json:"field2"`
	}

	i := 123
	intV := &AValue[int]{}
	b := Must(json.Marshal(i))
	if err := json.Unmarshal(b, intV); err != nil {
		t.Fatal("error unmarshaling value: ", err)
	}
	b2 := Must(json.Marshal(intV))
	if !bytes.Equal(b2, b) {
		t.Fatalf("bytes not equal: %v != %v", b2, b)
	}

	ts := testStruct{Field1: 1, Field2: "yes"}
	tsV := &AValue[testStruct]{}
	b = Must(json.Marshal(ts))
	if err := json.Unmarshal(b, tsV); err != nil {
		t.Fatal("error unmarshaling value: ", err)
	}
	b2 = Must(json.Marshal(tsV))
	if !bytes.Equal(b2, b) {
		t.Fatalf("bytes not equal: %v != %v", b2, b)
	}

	tsp := &testStruct{Field1: 1, Field2: "yes"}
	tspV := &AValue[*testStruct]{}
	b = Must(json.Marshal(tsp))
	if err := json.Unmarshal(b, tspV); err != nil {
		t.Fatal("error unmarshaling value: ", err)
	}
	b2 = Must(json.Marshal(tspV))
	if !bytes.Equal(b2, b) {
		t.Fatalf("bytes not equal: %v != %v", b2, b)
	}
}
