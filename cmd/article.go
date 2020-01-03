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
	"io/ioutil"
	"os"
	"path/filepath"
	"time"

	"github.com/fatih/color"
	"github.com/spf13/cobra"
)

var baseArticle = `---
title: My great title
description: This is a small summary of my article
slug: %s
---

The actual markdown content`

func init() {
	cmd := &cobra.Command{
		Use:     "article [slug]",
		Short:   "Create a new empty article",
		Long:    "Create a new article with the provided slug name",
		Example: "orion article my-new-article",
		Args:    cobra.ExactArgs(1),
		RunE:    newArticle,
	}

	rootCmd.AddCommand(cmd)

	cmd.Flags().BoolP("draft", "d", false, "A draft is prefixed with '_' and won't be built to HTML")
	cmd.Flags().BoolP("force", "f", false, "Overwrites the file if it already exists")
}

func newArticle(cmd *cobra.Command, args []string) error {
	force, _ := cmd.Flags().GetBool("force")
	draft, _ := cmd.Flags().GetBool("draft")

	slug := time.Now().Format("2006-01-02") + "-" + args[0]
	content := []byte(fmt.Sprintf(baseArticle, slug))

	filename := slug + ".md"
	if draft {
		filename = "_" + filename
	}
	path := filepath.Join("./articles/", filename)

	if _, err := os.Stat(path); !os.IsNotExist(err) && force == false {
		return errors.New(fmt.Sprintf("file %s already exists but you didn't use the --force flag to overwrite it", path))
	}

	if err := ioutil.WriteFile(path, content, 0755); err != nil {
		return err
	}

	color.Green("The article can be accessed on " + path)

	return nil
}
