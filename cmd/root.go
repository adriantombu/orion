// Package cmd regroups all the accessible commands of Orion
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

// Execute is the main command
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
