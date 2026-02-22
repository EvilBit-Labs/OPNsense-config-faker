package errors_test

import (
	"errors"
	"fmt"
	"testing"

	fakerErrors "github.com/EvilBit-Labs/opnsense-config-faker/internal/errors"
)

func TestConfigErrorSatisfiesErrorInterface(t *testing.T) {
	var err error = fakerErrors.NewConfigError("read", fmt.Errorf("file not found"))
	if err.Error() == "" {
		t.Fatal("ConfigError.Error() returned empty string")
	}
}

func TestConfigErrorUnwrap(t *testing.T) {
	inner := fmt.Errorf("underlying cause")
	err := fakerErrors.NewConfigError("write", inner)

	var target *fakerErrors.ConfigError
	if !errors.As(err, &target) {
		t.Fatal("errors.As failed to match ConfigError")
	}
	if target.Op != "write" {
		t.Fatalf("expected Op=%q, got %q", "write", target.Op)
	}
	if !errors.Is(err, inner) {
		t.Fatal("errors.Is failed to match underlying error")
	}
}

func TestConfigErrorMessage(t *testing.T) {
	err := fakerErrors.NewConfigError("parse", fmt.Errorf("invalid xml"))
	expected := "parse: invalid xml"
	if err.Error() != expected {
		t.Fatalf("expected %q, got %q", expected, err.Error())
	}
}

func TestVlanErrorSatisfiesErrorInterface(t *testing.T) {
	var err error = fakerErrors.NewVlanError(100, "id", "out of range")
	if err.Error() == "" {
		t.Fatal("VlanError.Error() returned empty string")
	}
}

func TestVlanErrorWithVlanID(t *testing.T) {
	err := fakerErrors.NewVlanError(42, "network", "not RFC 1918")
	expected := `vlan 42 [network]: not RFC 1918`
	if err.Error() != expected {
		t.Fatalf("expected %q, got %q", expected, err.Error())
	}
}

func TestVlanErrorWithoutVlanID(t *testing.T) {
	err := fakerErrors.NewVlanError(0, "pool", "exhausted")
	expected := `vlan error [pool]: exhausted`
	if err.Error() != expected {
		t.Fatalf("expected %q, got %q", expected, err.Error())
	}
}

func TestSeedErrorSatisfiesErrorInterface(t *testing.T) {
	var err error = fakerErrors.NewSeedError("data.csv", 3, "vlan_id", "invalid")
	if err.Error() == "" {
		t.Fatal("SeedError.Error() returned empty string")
	}
}

func TestSeedErrorMessage(t *testing.T) {
	err := fakerErrors.NewSeedError("input.csv", 5, "network", "malformed CIDR")
	expected := `seed error in input.csv row 5 col "network": malformed CIDR`
	if err.Error() != expected {
		t.Fatalf("expected %q, got %q", expected, err.Error())
	}
}
