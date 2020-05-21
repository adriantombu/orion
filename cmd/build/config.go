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

type paginationData struct {
	PrevTitle string
	PrevPath  string
	NextTitle string
	NextPath  string
}

type urlSet struct {
	XMLName  string `xml:"urlset"`
	Xmlns    string `xml:"xmlns,attr"`
	XmlnsXsi string `xml:"xmlns:xsi,attr"`
	Xsi      string `xml:"xsi:schemaLocation,attr"`
	Urls     []url  `xml:"url"`
}

type url struct {
	XMLName  string `xml:"url"`
	Loc      string `xml:"loc"`
	LastMod  string `xml:"lastmod"`
	Priority string `xml:"priority"`
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

type rss struct {
	Version       string `xml:"version,attr"`
	Title         string `xml:"channel>title"`
	Link          string `xml:"channel>link"`
	Description   string `xml:"channel>description"`
	LastBuildDate string `xml:"channel>lastBuildDate"`

	Item []rssItem `xml:"channel>item"`
}

type rssItem struct {
	Title       string `xml:"title"`
	Link        string `xml:"link"`
	Description string `xml:"description"`
	PubDate     string `xml:"pubDate"`
}

func init() {
	start = time.Now()
}
