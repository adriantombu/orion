// Package cmd regroups all the accessible commands of Orion
package cmd

import (
	"github.com/adriantombu/orion/cmd/build"
	"github.com/fatih/color"
	"github.com/spf13/cobra"
)

func init() {
	cmd := &cobra.Command{
		Use:   "build",
		Short: "Builds the blog for production",
		Long:  "This command will convert your markdown files to html and build everything to the /public directory",
		RunE:  generate,
	}

	rootCmd.AddCommand(cmd)
}

// generate starts the build
func generate(cmd *cobra.Command, args []string) error {
	color.Cyan("Building the Orion project for production")

	if err := build.Run(); err != nil {
		return err
	}

	color.Green("The production files can be accessed on the /public directory")

	return nil
}
