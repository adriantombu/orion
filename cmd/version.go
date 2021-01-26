// Package cmd regroups all the accessible commands of Orion
package cmd

import (
	"github.com/fatih/color"
	"github.com/spf13/cobra"
)

func init() {
	cmd := &cobra.Command{
		Use:   "version",
		Short: "Prints the current version of Orion",
		RunE:  version,
	}

	rootCmd.AddCommand(cmd)
}

// version prints the Orion build version
func version(_ *cobra.Command, _ []string) error {
	color.Cyan("Orion v0.1.6")

	return nil
}
