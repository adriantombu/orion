package build

import (
	"time"
)

var start time.Time
var articlesPath = "./articles"
var staticPath = "./static"
var themePath string
var buildPath string

type templateData struct {
	Title       string
	SiteName    string
	Canonical   string
	Content     string
	Pagination  paginationData
	FrontMatter frontMatter
}

type templateIndexData struct {
	Title       string
	Description string
	Canonical   string
	Articles    []frontMatter
}

type paginationData struct {
	PrevTitle string
	PrevPath  string
	NextTitle string
	NextPath  string
}

type frontMatter struct {
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

func init() {
	start = time.Now()
}
