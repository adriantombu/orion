package build

import (
	"bytes"
	"encoding/xml"
	"errors"
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
	"text/template"
	"time"

	"github.com/fatih/color"
	"github.com/gomarkdown/markdown"
	"github.com/otiai10/copy"
	"github.com/spf13/viper"
	"gopkg.in/yaml.v2"
)

func Run() error {
	themePath = fmt.Sprintf("./themes/%s", viper.GetString("theme"))
	buildPath = viper.GetString("build_path")
	if buildPath == "" {
		buildPath = "./public"
	}

	if err := resetBuildDirectory(); err != nil {
		return err
	}

	articles, err := getArticles()
	if err != nil {
		return err
	}

	for i := 0; i < len(articles); i++ {
		if err := writeArticles(articles, i); err != nil {
			return err
		}
	}

	if err := copyAssets(); err != nil {
		return err
	}

	if err := generateSitemap(articles); err != nil {
		return err
	}

	if err := generateRss(articles); err != nil {
		return err
	}

	color.Cyan("Total duration %v\n", time.Since(start))

	return nil
}

func resetBuildDirectory() error {
	if err := os.RemoveAll(buildPath); err != nil {
		return errors.New("could not reset build path")
	}

	if err := os.Mkdir(buildPath, os.ModePerm); err != nil {
		return errors.New("could not create build path")
	}

	return nil
}

func getArticles() ([]string, error) {
	var articles []string

	files, err := ioutil.ReadDir(articlesPath)
	if err != nil {
		return articles, err
	}

	for _, file := range files {
		fileName := file.Name()

		if !file.IsDir() && filepath.Ext(fileName) == ".md" && strings.HasPrefix(fileName, "_") == false {
			articles = append(articles, fileName)
		}
	}

	sort.Sort(sort.Reverse(sort.StringSlice(articles)))

	return articles, nil
}

func getHTML(filename string) (frontMatter, string, error) {
	var fm frontMatter
	var html string

	path := filepath.Join(articlesPath, filename)
	data, err := ioutil.ReadFile(path)
	if err != nil {
		return fm, html, err
	}

	mkData := strings.TrimSpace(string(data))
	values := strings.SplitN(mkData, "---", 3)
	if err = yaml.Unmarshal([]byte(values[1]), &fm); err != nil {
		return fm, html, err
	}
	html = string(markdown.ToHTML([]byte(values[2]), nil, nil))
	populateFm(&fm)

	return fm, html, nil
}

func populateFm(fm *frontMatter) {
	if fm.Robots == "" {
		if robots := viper.GetString("robots"); robots != "" {
			fm.Robots = robots
		}
	}

	if fm.OpenGraph.Type == "" {
		if ogType := viper.GetString("opengraph.type"); ogType != "" {
			fm.OpenGraph.Type = ogType
		}
	}

	if fm.Twitter.Card == "" {
		if twCard := viper.GetString("twitter.card"); twCard != "" {
			fm.Twitter.Card = twCard
		}
	}

	if fm.Twitter.Site == "" {
		if twSite := viper.GetString("twitter.site"); twSite != "" {
			fm.Twitter.Site = twSite
		}
	}

	if fm.Twitter.Creator == "" {
		if twCreator := viper.GetString("twitter.creator"); twCreator != "" {
			fm.Twitter.Creator = twCreator
		}
	}
}

func getTitle(fm frontMatter, html string) (string, error) {
	if fm.Title != "" {
		return fm.Title, nil
	}

	re, err := regexp.Compile(`<h1>(.*)</h1>`)
	if err != nil {
		return "", err
	}

	return re.FindStringSubmatch(html)[1], nil
}

func getPage(html string, title string, pagination paginationData, filename string, fm frontMatter) string {
	t, err := template.ParseFiles(filepath.Join(themePath, "template.html"))
	if err != nil {
		log.Fatal(err)
	}

	canonical := viper.GetString("base_url") + filename
	if fm.Canonical != "" {
		canonical = fm.Canonical
	}

	var tpl bytes.Buffer
	err = t.Execute(&tpl, templateData{
		Title:       title,
		SiteName:    viper.GetString("site_name"),
		Canonical:   canonical,
		Content:     html,
		Pagination:  pagination,
		FrontMatter: fm,
	})
	if err != nil {
		log.Fatal(err)
	}

	return tpl.String()
}

func getPagination(files []string, current int) (paginationData, error) {
	var data paginationData

	if current != 0 {
		fm, prevHTML, err := getHTML(files[current-1])
		if err != nil {
			return data, err
		}

		prevTitle, errT := getTitle(fm, prevHTML)
		if errT != nil {
			return data, errT
		}

		data.PrevTitle = prevTitle
		data.PrevPath = strings.Replace(files[current-1], ".md", ".html", 1)

		if current-1 == 0 {
			data.PrevPath = "index.html"
		}
	}

	if current != len(files)-1 && current+1 <= len(files)-1 {
		fm, nextHTML, err := getHTML(files[current+1])
		if err != nil {
			return data, err
		}

		nextTitle, errT := getTitle(fm, nextHTML)
		if errT != nil {
			return data, errT
		}

		data.NextTitle = nextTitle
		data.NextPath = strings.Replace(files[current+1], ".md", ".html", 1)
	}

	return data, nil
}

func writeArticles(articles []string, i int) error {
	file := articles[i]

	color.Magenta("Generating file %s...", file)

	fm, html, err := getHTML(file)
	if err != nil {
		return err
	}

	if len(fm.Description) > 160 {
		color.Yellow("The description exceeds 160 characters, you should consider shortening it for SEO performances")
	}

	title, errT := getTitle(fm, html)
	if errT != nil {
		return errT
	}

	if len(title) > 70 {
		color.Yellow("The title exceeds 70 characters, you should consider shortening it for SEO performances")
	}

	pagination, errP := getPagination(articles, i)
	if errP != nil {
		return errP
	}

	filename := strings.Replace(file, ".md", ".html", 1)
	if fm.Slug != "" {
		filename = fm.Slug + ".html"
	}
	page := getPage(html, title, pagination, filename, fm)

	if i == 0 {
		if err := ioutil.WriteFile(filepath.Join(buildPath, filename), []byte(page), 0755); err != nil {
			return err
		}

		filename = "index.html"
	}

	if err := ioutil.WriteFile(filepath.Join(buildPath, filename), []byte(page), 0755); err != nil {
		return err
	}

	return nil
}

func copyAssets() error {
	if err := copy.Copy(filepath.Join(themePath, "favicon.png"), filepath.Join(buildPath, "favicon.png")); err != nil {
		return err
	}

	if err := copy.Copy(filepath.Join(themePath, "style.css"), filepath.Join(buildPath, "style.css")); err != nil {
		return err
	}

	if err := copy.Copy(staticPath, buildPath); err != nil {
		return err
	}

	return nil
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

func generateRss(articles []string) error {
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
			return err
		}

		publishedAt, errDate := time.Parse("2006-01-02", fm.PublishedAt)
		if errDate != nil {
			return errDate
		}

		rssStruct.Item = append(rssStruct.Item, rssItem{
			Title:       fm.Title,
			Description: html,
			Link:        viper.GetString("base_url") + filename,
			PubDate:     publishedAt.Format(time.RFC1123Z),
		})
	}

	data, _ := xml.MarshalIndent(rssStruct, "", "    ")
	rssFeed := []byte(xml.Header + string(data))

	if err := ioutil.WriteFile(filepath.Join(buildPath, "rss.xml"), rssFeed, 0644); err != nil {
		return err
	}

	return nil
}
