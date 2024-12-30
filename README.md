# utils
Just some utilities that I often find myself wanting and recreating in various languages.

# TODO

## Rust
- [X] die macro
- [X] OrDie trait for Result/Option
    - [ ] PResult
- [X] MapValue for any item that takes an immutable value and allows mutation in a closure, returning the new value.
    - [ ] Tests
- [ ] presult other io and common things (e.g., tokio and futures respective Read/Write)
- [ ] Docs
    - [x] presult
        - [ ] pio
- [ ] Tests
    - [ ] presult
        - [ ] pio
    - [ ] OrDie

## Go
- [ ] Tests
    - [ ] Use black box testing?
- [x] DeferedCloser to add close funcs to possibly defer (based on DeferClose)
- [x] DeferedFunc to add funcs to possibly defer (based on DeferFunc)
- [x] Slice and SlicePtr wrapper structs
    - [x] Slice First/Last funcs
- [ ] Various encodings (e.g., JSON) for Mutexes and AValue
    - [x] (Un)MarshalJSON
- [x] Change Error field in ErrorValue from Error to Err?
- [ ] Func to create listener that has REUSEADDR set
- [x] Map maps, filters, etc.
    - [x] Create Map, Set, Slice with capacity
