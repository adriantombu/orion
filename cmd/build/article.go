package build

import (
	"bytes"
	"fmt"
	"io/ioutil"
	"path/filepath"
	"regexp"
	"strings"
	"text/template"

	"github.com/fatih/color"
	"github.com/gomarkdown/markdown"
	"github.com/spf13/viper"
	"gopkg.in/yaml.v2"
)

type articles []article
type article struct {
	MarkdownPath string
	HTMLPath     string
	Data         []string
	Metadata     metaData
	HTML         string
	Title        string
	Canonical    string
	Template     string
}

type metaData struct {
	Title       string `yaml:"title"`
	Description string `yaml:"description"`
	Slug        string `yaml:"slug"`
	Canonical   string `yaml:"canonical"`
	Robots      string `yaml:"robots"`
	PublishedAt string `yaml:"published_at"`

	OpenGraph struct {
		Type  string `yaml:"type"`
		Image string `yaml:"image"`
	}

	Twitter struct {
		Card    string `yaml:"card"`
		Site    string `yaml:"site"`
		Creator string `yaml:"creator"`
	}
}

type templateArticle struct {
	Title     string
	SiteName  string
	Canonical string
	Content   string
	Metadata  metaData
}

// parse get all the data needed to populate all the fields of the article
func (article article) parse() (article, error) {
	article.Data = article.getData()
	article.Metadata = article.getMetadata()
	article.HTMLPath = article.getHTMLPath()
	article.Canonical = article.getCanonical()
	article.HTML = article.getHTML()
	article.Title = article.getTitle()
	article.Template = article.getTemplate()

	return article, nil
}

// getData parse the file and returns raw data
func (article article) getData() []string {
	var res []string

	path := filepath.Join(articlesPath, article.MarkdownPath)
	raw, err := ioutil.ReadFile(path)
	if err != nil {
		color.Red(err.Error())
		return res
	}

	data := strings.TrimSpace(string(raw))
	return strings.SplitN(data, "---", 3)
}

// getMetadata parses the raw data and extracts the frontMatter values
func (article article) getMetadata() metaData {
	var metaData metaData

	if err := yaml.Unmarshal([]byte(article.Data[1]), &metaData); err != nil {
		color.Red(err.Error())
		return metaData
	}

	if metaData.Robots == "" {
		if robots := viper.GetString("robots"); robots != "" {
			metaData.Robots = robots
		}
	}

	if metaData.OpenGraph.Type == "" {
		if ogType := viper.GetString("opengraph.type"); ogType != "" {
			metaData.OpenGraph.Type = ogType
		}
	}

	if metaData.Twitter.Card == "" {
		if twCard := viper.GetString("twitter.card"); twCard != "" {
			metaData.Twitter.Card = twCard
		}
	}

	if metaData.Twitter.Site == "" {
		if twSite := viper.GetString("twitter.site"); twSite != "" {
			metaData.Twitter.Site = twSite
		}
	}

	if metaData.Twitter.Creator == "" {
		if twCreator := viper.GetString("twitter.creator"); twCreator != "" {
			metaData.Twitter.Creator = twCreator
		}
	}

	return metaData
}

// getHTMLPath returns the name of the HTML file
func (article article) getHTMLPath() string {
	htmlPath := strings.Replace(article.MarkdownPath, ".md", ".html", 1)

	if article.Metadata.Slug != "" {
		return fmt.Sprintf("%s.html", article.Metadata.Slug)
	}

	return fmt.Sprintf("%s", htmlPath)
}

// getCanonical returns the canonical url
func (article article) getCanonical() string {
	if article.Metadata.Canonical != "" {
		return article.Metadata.Canonical
	}

	return fmt.Sprintf("%s%s", viper.GetString("base_url"), article.HTMLPath)
}

// getHTML returns the HTML content of the article
func (article article) getHTML() string {
	return string(markdown.ToHTML([]byte(article.Data[2]), nil, nil))
}

// getTitle returns the title of the article
func (article article) getTitle() string {
	if article.Metadata.Title != "" {
		return article.Metadata.Title
	}

	re, err := regexp.Compile(`<h1>(.*)</h1>`)
	if err != nil {
		color.Red(err.Error())
		return ""
	}

	return re.FindStringSubmatch(article.HTML)[1]
}

// getTemplate returns the template of the article
func (article article) getTemplate() string {
	t, err := template.ParseFiles(filepath.Join(themePath, "article.html"))
	if err != nil {
		color.Red(err.Error())
		return ""
	}

	var tpl bytes.Buffer
	err = t.Execute(&tpl, templateArticle{
		Title:     article.Title,
		SiteName:  viper.GetString("site_name"),
		Canonical: article.Canonical,
		Content:   article.HTML,
		Metadata:  article.Metadata,
	})
	if err != nil {
		color.Red(err.Error())
		return ""
	}

	return tpl.String()
}

// write creates the HTML file of the article
func (article article) write() {
	defer wg.Done()

	color.Magenta("Generating file %s...", article.MarkdownPath)

	if len(article.Metadata.Description) > 160 {
		color.Yellow("The description exceeds 160 characters, you should consider shortening it for SEO performances")
	}

	if len(article.Title) > 70 {
		color.Yellow("The title exceeds 70 characters, you should consider shortening it for SEO performances")
	}

	if err := ioutil.WriteFile(filepath.Join(buildPath, article.HTMLPath), []byte(article.Template), 0755); err != nil {
		color.Red(err.Error())
	}
}
