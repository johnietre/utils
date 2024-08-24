# utils
Just some utilities that I often find myself wanting and recreating in various languages.

# TODO

## Rust
- [ ] MapValue for any item that takes an immutable value and allows mutation in a closure, returning the new value.

## Go
- [ ] Tests
- [x] DeferedCloser to add close funcs to possibly defer (based on DeferClose)
- [x] DeferedFunc to add funcs to possibly defer (based on DeferFunc)
- [x] Slice and SlicePtr wrapper structs
- [ ] Various encodings (e.g., JSON) for Mutexes and AValue
    - [x] (Un)MarshalJSON
- [ ] Change Error field in ErrorValue from Error to Err?
- [ ] Func to create listener that has REUSEADDR set
- [ ] Map maps, filters, etc.
    - [ ] Create Map, Set, Slice with capacity
