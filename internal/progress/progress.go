// Package progress provides progress indication control for CLI operations.
package progress

// noopActive tracks whether progress output has been disabled.
var noopActive bool //nolint:gochecknoglobals // module-level toggle

// Noop disables progress output for the current process.
// Call this when TERM=dumb or --quiet is set.
func Noop() {
	noopActive = true
}

// IsNoop returns true when progress output has been disabled.
func IsNoop() bool {
	return noopActive
}
