/*
Copyright © 2020 Adrian Tombu <adrian@otso.fr>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <http://www.gnu.org/licenses/>.
*/
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
		RunE:  buildBlog,
	}

	rootCmd.AddCommand(cmd)
}

func buildBlog(cmd *cobra.Command, args []string) error {
	color.Cyan("Building the Orion project for production")

	if err := build.Run(); err != nil {
		return err
	}

	color.Green("The production files can be accessed on the /public directory")

	return nil
}
