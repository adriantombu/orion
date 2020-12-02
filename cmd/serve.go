// Package cmd regroups all the accessible commands of Orion
package cmd

import (
	"net/http"

	"github.com/adriantombu/orion/cmd/build"
	"github.com/fatih/color"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

func init() {
	cmd := &cobra.Command{
		Use:     "serve",
		Short:   "Build and serve your blog",
		Long:    "This command builds your website and serve your blog on the provided local url",
		Example: "orion serve -p 1234",
		RunE:    serve,
	}

	rootCmd.AddCommand(cmd)

	cmd.Flags().StringP("port", "p", "1337", "Change the port")
}

// serve builds the blog and creates a temporary server to navigate the pages
func serve(cmd *cobra.Command, args []string) error {
	cobra.OnInitialize(initConfig)

	port, _ := cmd.Flags().GetString("port")

	if err := build.Run(); err != nil {
		return err
	}

	buildPath := viper.GetString("build_path")
	if buildPath == "" {
		buildPath = "./public"
	}

	http.Handle("/", http.FileServer(http.Dir(buildPath)))

	color.Green("You can access your blog on http://localhost:%s", port)

	if err := http.ListenAndServe(":"+port, nil); err != nil {
		return err
	}

	return nil
}
