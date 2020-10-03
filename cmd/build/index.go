package build

import (
	"bytes"
	"io/ioutil"
	"path/filepath"
	"text/template"

	"github.com/fatih/color"
	"github.com/spf13/viper"
)

type templateIndex struct {
	Title       string
	Description string
	Canonical   string
	Articles    articles
}

// generateIndex creates the index.html page that displays a list of articles
func generateIndex(articles articles) {
	defer wg.Done()

	color.Magenta("Generating file index.html...")

	t, err := template.ParseFiles(filepath.Join(themePath, "index.html"))
	if err != nil {
		color.Red(err.Error())
		return
	}

	var tpl bytes.Buffer
	err = t.Execute(&tpl, templateIndex{
		Title:       viper.GetString("site_name"),
		Description: viper.GetString("description"),
		Canonical:   viper.GetString("base_url"),
		Articles:    articles,
	})
	if err != nil {
		color.Red(err.Error())
		return
	}

	page := tpl.String()

	if err := ioutil.WriteFile(filepath.Join(buildPath, "index.html"), []byte(page), 0755); err != nil {
		color.Red(err.Error())
		return
	}
}
