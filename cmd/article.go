// Package cmd regroups all the accessible commands of Orion
package cmd

import (
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
published_at: 2020-05-21
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

// newArticle creates a new Markdown article
func newArticle(cmd *cobra.Command, args []string) error {
	force, _ := cmd.Flags().GetBool("force")
	draft, _ := cmd.Flags().GetBool("draft")

	slug := fmt.Sprintf("%s-%s", time.Now().Format("2006-01-02"), args[0])
	content := []byte(fmt.Sprintf(baseArticle, slug))

	filename := fmt.Sprintf("%s.md", slug)
	if draft {
		filename = fmt.Sprintf("_%s", filename)
	}

	path := filepath.Join("./articles/", filename)
	if _, err := os.Stat(path); !os.IsNotExist(err) && force == false {
		return fmt.Errorf("file %s already exists but you didn't use the --force flag to overwrite it", path)
	}

	if err := ioutil.WriteFile(path, content, 0755); err != nil {
		return err
	}

	color.Green("The article can be accessed on %s", path)

	return nil
}
