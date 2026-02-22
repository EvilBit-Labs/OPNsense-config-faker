package generator_test

import (
	"math/rand/v2"
	"testing"

	"github.com/EvilBit-Labs/opnsense-config-faker/internal/generator"
)

func TestDepartmentCount(t *testing.T) {
	if len(generator.Departments) != 20 {
		t.Fatalf("expected 20 departments, got %d", len(generator.Departments))
	}
}

func TestDepartmentNamesAreDistinct(t *testing.T) {
	seen := make(map[string]bool)
	for _, dept := range generator.Departments {
		if seen[dept.Name] {
			t.Fatalf("duplicate department name: %s", dept.Name)
		}
		seen[dept.Name] = true
	}
}

func TestDHCPLeaseTimesValid(t *testing.T) {
	validTimes := map[string]bool{
		"4h": true, "8h": true, "12h": true,
		"16h": true, "20h": true, "24h": true,
	}
	for _, dept := range generator.Departments {
		if dept.DHCPLeaseTime == "" {
			t.Fatalf("department %s has empty DHCP lease time", dept.Name)
		}
		if !validTimes[dept.DHCPLeaseTime] {
			t.Fatalf("department %s has invalid DHCP lease time %q", dept.Name, dept.DHCPLeaseTime)
		}
	}
}

func TestDHCPLeaseTimeRange(t *testing.T) {
	// Verify all 6 lease time values are represented
	seen := make(map[string]bool)
	for _, dept := range generator.Departments {
		seen[dept.DHCPLeaseTime] = true
	}
	expected := []string{"4h", "8h", "12h", "16h", "20h", "24h"}
	for _, lt := range expected {
		if !seen[lt] {
			t.Errorf("lease time %s not represented in departments", lt)
		}
	}
}

func TestNoAdjacentDuplicateLeaseTime(t *testing.T) {
	for i := 1; i < len(generator.Departments); i++ {
		if generator.Departments[i].DHCPLeaseTime == generator.Departments[i-1].DHCPLeaseTime {
			t.Errorf("adjacent departments %s and %s share lease time %s",
				generator.Departments[i-1].Name, generator.Departments[i].Name,
				generator.Departments[i].DHCPLeaseTime)
		}
	}
}

func TestRandomDepartmentDeterministic(t *testing.T) {
	rng1 := rand.New(rand.NewPCG(42, 0))
	rng2 := rand.New(rand.NewPCG(42, 0))

	for range 10 {
		d1 := generator.RandomDepartment(rng1)
		d2 := generator.RandomDepartment(rng2)
		if d1 != d2 {
			t.Fatalf("same seed produced different results: %v vs %v", d1, d2)
		}
	}
}

func TestRandomDepartmentInList(t *testing.T) {
	rng := rand.New(rand.NewPCG(99, 0))
	valid := make(map[string]bool)
	for _, d := range generator.Departments {
		valid[d.Name] = true
	}

	for i := range 20 {
		d := generator.RandomDepartment(rng)
		if !valid[d.Name] {
			t.Errorf("iteration %d: random department %q not in list", i, d.Name)
		}
	}
}

func TestAllDepartmentsReturnsCopy(t *testing.T) {
	all := generator.AllDepartments()
	if len(all) != 20 {
		t.Fatalf("expected 20, got %d", len(all))
	}
	// Mutating the copy should not affect the original
	all[0].Name = "MUTATED"
	if generator.Departments[0].Name == "MUTATED" {
		t.Fatal("AllDepartments returned a reference, not a copy")
	}
}
