// Package main is the entry point for the opnsense-config-faker CLI tool.
package main

import (
	"os"

	"github.com/EvilBit-Labs/opnsense-config-faker/cmd"
)

func main() {
	if err := cmd.Execute(); err != nil {
		os.Exit(1)
	}
}
