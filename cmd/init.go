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
	"errors"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"
	"text/tabwriter"

	"github.com/fatih/color"
	"github.com/markbates/pkger"
	"github.com/spf13/cobra"
)

func init() {
	cmd := &cobra.Command{
		Use:     "init [path]",
		Short:   "Create a new blog",
		Long:    "Create a new blog in provided path. It will be a skeleton with the base theme and a few generic articles.",
		Example: "orion init blog-path",
		Args:    cobra.ExactArgs(1),
		RunE:    newBlog,
	}

	rootCmd.AddCommand(cmd)

	cmd.Flags().BoolP("force", "f", false, "Initialize inside a non-empty directory")
}

func newBlog(cmd *cobra.Command, args []string) error {
	color.Cyan("Initializing a new Orion project")

	force, _ := cmd.Flags().GetBool("force")
	basePath := args[0]
	path, err := filepath.Abs(basePath)
	if err != nil {
		return err
	}

	if _, err := os.Stat(path); !os.IsNotExist(err) && force == false {
		return errors.New(fmt.Sprintf("directory %s already exists but you didn't use the --force flag", path))
	}

	// Create base folder
	if err := os.MkdirAll(basePath, 0755); err != nil {
		return err
	}

	w := tabwriter.NewWriter(os.Stdout, 0, 0, 0, ' ', tabwriter.Debug)
	defer w.Flush()

	errWalk := pkger.Walk("/cmd/init", func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}

		cleanPath := strings.Split(path, ":/cmd/init")[1]

		if base := filepath.Base(cleanPath); base == "." || base == ".DS_Store" {
			return nil
		}

		color.Magenta("Creating %s...", basePath+cleanPath)

		if info.IsDir() {
			if err := os.MkdirAll(filepath.Join(basePath, cleanPath), 0755); err != nil {
				return err
			}

			return nil
		}

		dst, errDst := os.Create(filepath.Join(basePath, cleanPath))
		if errDst != nil {
			return err
		}
		defer dst.Close()

		src, errSrc := pkger.Open(path)
		if errSrc != nil {
			return errSrc
		}
		defer src.Close()

		if _, err := io.Copy(dst, src); err != nil {
			return err
		}

		return nil
	})
	if errWalk != nil {
		return errWalk
	}

	color.Green("A new project was created on the following path %s", path)

	return nil
}
