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
	"os"

	"github.com/fatih/color"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

var rootCmd = &cobra.Command{
	Use:           "orion",
	Short:         "A simple static blog generator",
	Long:          "Write your article in Markdown and build them into a static HTML website",
	SilenceErrors: true,
}

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		color.Red(err.Error())

		os.Exit(1)
	}
}

func init() {
	cobra.OnInitialize(initConfig)

	rootCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}

func initConfig() {
	viper.SetConfigName("config")
	viper.AddConfigPath(".")
	viper.AutomaticEnv()

	if err := viper.ReadInConfig(); err != nil {
		color.Red("Config file not found: %s", viper.ConfigFileUsed())
	}
}
