package build

import (
	"encoding/xml"
	"io/ioutil"
	"path/filepath"
	"strings"
	"sync"
	"time"

	"github.com/fatih/color"
	"github.com/spf13/viper"
)

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
	Image       string `xml:"image"`
	PubDate     string `xml:"pubDate"`
}

func generateRss(articles []string, wg *sync.WaitGroup) {
	defer wg.Done()

	rssStruct := &rss{
		Version:       "2.0",
		Title:         viper.GetString("site_name"),
		Link:          viper.GetString("base_url"),
		Description:   viper.GetString("description"),
		LastBuildDate: time.Now().Format(time.RFC1123Z),
	}

	for i := 0; i < len(articles); i++ {
		file := articles[i]
		filename := strings.Replace(file, ".md", ".html", 1)

		fm, html, err := getHTML(file)
		if err != nil {
			color.Red(err.Error())
			return
		}

		publishedAt, errDate := time.Parse("2006-01-02", fm.PublishedAt)
		if errDate != nil {
			color.Red(errDate.Error())
			return
		}

		rssStruct.Item = append(rssStruct.Item, rssItem{
			Title:       fm.Title,
			Description: html,
			Link:        viper.GetString("base_url") + filename,
			Image:       fm.OpenGraph.Image,
			PubDate:     publishedAt.Format(time.RFC1123Z),
		})
	}

	data, _ := xml.MarshalIndent(rssStruct, "", "    ")
	rssFeed := []byte(xml.Header + string(data))

	if err := ioutil.WriteFile(filepath.Join(buildPath, "rss.xml"), rssFeed, 0644); err != nil {
		color.Red(err.Error())
		return
	}
}
