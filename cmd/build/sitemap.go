package build

import (
	"encoding/xml"
	"fmt"
	"io/ioutil"
	"os"
	"path/filepath"
	"strings"
	"time"

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

func generateSitemap(articles []string) error {
	urlSet := &urlSet{
		Xmlns:    "http://www.sitemaps.org/schemas/sitemap/0.9",
		XmlnsXsi: "http://www.w3.org/2001/XMLSchema-instance",
		Xsi:      "http://www.sitemaps.org/schemas/sitemap/0.9 http://www.sitemaps.org/schemas/sitemap/0.9/sitemap.xsd",
	}

	priority := 1.00

	for i := 0; i < len(articles); i++ {
		file := articles[i]
		filename := strings.Replace(file, ".md", ".html", 1)

		fileStat, err := os.Stat(filepath.Join(articlesPath, file))
		if err != nil {
			return err
		}

		if i == 0 {
			urlSet.Urls = append(urlSet.Urls, url{
				Loc:      viper.GetString("base_url") + "index.html",
				LastMod:  fileStat.ModTime().Format(time.RFC3339),
				Priority: fmt.Sprintf("%.2f", priority),
			})
		}

		urlSet.Urls = append(urlSet.Urls, url{
			Loc:      viper.GetString("base_url") + filename,
			LastMod:  fileStat.ModTime().Format(time.RFC3339),
			Priority: fmt.Sprintf("%.2f", priority),
		})

		priority = priority * 0.8
	}

	data, _ := xml.MarshalIndent(urlSet, "", "    ")
	sitemap := []byte(xml.Header + string(data))

	if err := ioutil.WriteFile(filepath.Join(buildPath, "sitemap.xml"), sitemap, 0644); err != nil {
		return err
	}

	return nil
}
