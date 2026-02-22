// Package cmd implements the CLI commands for opnsense-config-faker.
package cmd

import (
	"context"
	"os"

	"github.com/EvilBit-Labs/opnsense-config-faker/internal/progress"
	"github.com/charmbracelet/fang"
	"github.com/spf13/cobra"
)

var (
	quiet   bool //nolint:gochecknoglobals // CLI flag
	noColor bool //nolint:gochecknoglobals // CLI flag
)

var rootCmd = &cobra.Command{ //nolint:gochecknoglobals // Cobra root command
	Use:   "opnsense-config-faker",
	Short: "Generate realistic OPNsense configuration files with faked data",
	Long: `opnsense-config-faker generates valid OPNsense config.xml files populated
with realistic faked network data including VLANs, interfaces, DHCP, NAT,
firewall policies, CARP VIPs, and RADIUS users.

Built for network operators and automation engineers who need realistic
OPNsense test configurations.`,
	PersistentPreRun: func(_ *cobra.Command, _ []string) {
		// Detect TERM=dumb for CI/automation environments
		if os.Getenv("TERM") == "dumb" {
			quiet = true
			progress.Noop()
		}
		if quiet {
			progress.Noop()
		}
	},
}

func init() {
	rootCmd.PersistentFlags().BoolVarP(&quiet, "quiet", "q", false, "Suppress non-essential output")
	rootCmd.PersistentFlags().BoolVar(&noColor, "no-color", false, "Disable colored output")

	// Bind NO_COLOR env var
	if os.Getenv("NO_COLOR") != "" {
		noColor = true
	}
}

// Execute runs the root command via fang.
func Execute() error {
	return fang.Execute(context.Background(), rootCmd)
}
