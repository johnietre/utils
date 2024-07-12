package utils

// CloneMap clonse a map.
func CloneMap[K comparable, V any](m map[K]V) map[K]V {
	nm := make(map[K]V, len(m))
	for k, v := range m {
		nm[k] = v
	}
	return nm
}

// CloneMapInto copies the key/value pairs from `src` into `dst`, returning
// `dst` (not a new map). Panics if `dst` is nil.
func CloneMapInto[K comparable, V any](dst, src map[K]V) map[K]V {
	for k, v := range src {
		dst[k] = v
	}
	return dst
}
