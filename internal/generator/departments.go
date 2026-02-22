// Package generator provides data generation utilities for OPNsense configurations.
package generator

import "math/rand/v2"

// Department represents a network department with its DHCP lease configuration.
type Department struct {
	Name          string
	DHCPLeaseTime string
}

// Departments contains the 20 standard department definitions with DHCP lease times
// spanning 4h-24h. No two adjacent departments share the same lease time.
var Departments = [20]Department{
	{Name: "Sales", DHCPLeaseTime: "8h"},
	{Name: "IT", DHCPLeaseTime: "24h"},
	{Name: "HR", DHCPLeaseTime: "4h"},
	{Name: "Finance", DHCPLeaseTime: "12h"},
	{Name: "Marketing", DHCPLeaseTime: "8h"},
	{Name: "Operations", DHCPLeaseTime: "16h"},
	{Name: "Engineering", DHCPLeaseTime: "24h"},
	{Name: "Support", DHCPLeaseTime: "4h"},
	{Name: "Legal", DHCPLeaseTime: "12h"},
	{Name: "Procurement", DHCPLeaseTime: "20h"},
	{Name: "Security", DHCPLeaseTime: "8h"},
	{Name: "Development", DHCPLeaseTime: "24h"},
	{Name: "QA", DHCPLeaseTime: "4h"},
	{Name: "Research", DHCPLeaseTime: "16h"},
	{Name: "Training", DHCPLeaseTime: "8h"},
	{Name: "Management", DHCPLeaseTime: "20h"},
	{Name: "Accounting", DHCPLeaseTime: "12h"},
	{Name: "Customer Service", DHCPLeaseTime: "4h"},
	{Name: "Logistics", DHCPLeaseTime: "16h"},
	{Name: "Production", DHCPLeaseTime: "24h"},
}

// RandomDepartment returns a random department from the list.
func RandomDepartment(rng *rand.Rand) Department {
	return Departments[rng.IntN(len(Departments))]
}

// AllDepartments returns a copy of the departments slice to prevent external mutation.
func AllDepartments() []Department {
	result := make([]Department, len(Departments))
	copy(result, Departments[:])
	return result
}
