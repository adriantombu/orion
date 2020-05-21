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

func serve(cmd *cobra.Command, args []string) error {
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
