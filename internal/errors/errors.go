// Package errors provides structured error types for the OPNsense Config Faker.
package errors

import "fmt"

// ConfigError wraps I/O, XML, and general configuration failures.
type ConfigError struct {
	Op  string // operation name
	Err error  // underlying cause
}

func (e *ConfigError) Error() string {
	return fmt.Sprintf("%s: %v", e.Op, e.Err)
}

func (e *ConfigError) Unwrap() error {
	return e.Err
}

// NewConfigError creates a new ConfigError.
func NewConfigError(op string, err error) *ConfigError {
	return &ConfigError{Op: op, Err: err}
}

// VlanError represents VLAN-specific failures.
type VlanError struct {
	VlanID int    // 0 when not applicable
	Field  string // field that caused the error
	Msg    string // human-readable message
}

func (e *VlanError) Error() string {
	if e.VlanID == 0 {
		return fmt.Sprintf("vlan error [%s]: %s", e.Field, e.Msg)
	}
	return fmt.Sprintf("vlan %d [%s]: %s", e.VlanID, e.Field, e.Msg)
}

// NewVlanError creates a new VlanError.
func NewVlanError(vlanID int, field, msg string) *VlanError {
	return &VlanError{VlanID: vlanID, Field: field, Msg: msg}
}

// SeedError represents CSV seed ingestion failures.
type SeedError struct {
	File string // source file path
	Row  int    // row number (0-indexed)
	Col  string // column name
	Msg  string // human-readable message
}

func (e *SeedError) Error() string {
	return fmt.Sprintf("seed error in %s row %d col %q: %s", e.File, e.Row, e.Col, e.Msg)
}

// NewSeedError creates a new SeedError.
func NewSeedError(file string, row int, col, msg string) *SeedError {
	return &SeedError{File: file, Row: row, Col: col, Msg: msg}
}
