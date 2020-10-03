package build

import (
	"encoding/xml"
	"fmt"
	"io/ioutil"
	"path/filepath"
	"time"

	"github.com/fatih/color"
	"github.com/spf13/viper"
)

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

// generateSitemap creates a sitemap for SEO purposes
func generateSitemap(articles articles) {
	defer wg.Done()

	urlSet := urlSet{
		Xmlns:    "http://www.sitemaps.org/schemas/sitemap/0.9",
		XmlnsXsi: "http://www.w3.org/2001/XMLSchema-instance",
		Xsi:      "http://www.sitemaps.org/schemas/sitemap/0.9 http://www.sitemaps.org/schemas/sitemap/0.9/sitemap.xsd",
	}

	priority := 1.00
	urlSet.Urls = append(urlSet.Urls, url{
		Loc:      fmt.Sprintf("%s%s", viper.GetString("base_url"), "index.html"),
		LastMod:  time.Now().Format(time.RFC3339),
		Priority: fmt.Sprintf("%.2f", priority),
	})

	for _, article := range articles {
		publishedAt, errDate := time.Parse("2006-01-02", article.Metadata.PublishedAt)
		if errDate != nil {
			color.Red(errDate.Error())
			return
		}

		urlSet.Urls = append(urlSet.Urls, url{
			Loc:      fmt.Sprintf("%s%s", viper.GetString("base_url"), article.HTMLPath),
			LastMod:  publishedAt.Format(time.RFC3339),
			Priority: fmt.Sprintf("%.2f", priority),
		})

		priority = priority * 0.9
	}

	data, _ := xml.MarshalIndent(urlSet, "", "    ")
	sitemap := []byte(xml.Header + string(data))

	if err := ioutil.WriteFile(filepath.Join(buildPath, "sitemap.xml"), sitemap, 0644); err != nil {
		color.Red(err.Error())
		return
	}
}
