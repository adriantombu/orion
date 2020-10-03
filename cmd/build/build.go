// Package build handles the compilation of the files
package build

import (
	"errors"
	"fmt"
	"io/ioutil"
	"os"
	"path/filepath"
	"sort"
	"strings"
	"sync"
	"time"

	"github.com/fatih/color"

	"github.com/spf13/viper"
)

var wg sync.WaitGroup
var start time.Time
var articlesPath = "./articles"
var staticPath = "./static"
var themePath string
var buildPath string

func init() {
	start = time.Now()
}

// Run builds the HTML from Markdown files, copy the assets and generates a sitemap and a RSS feed
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

	wg.Add(len(articles))
	for _, article := range articles {
		go article.write()
	}

	wg.Add(4)
	go generateIndex(articles)
	go copyAssets()
	go generateSitemap(articles)
	go generateRss(articles)

	wg.Wait()

	color.Cyan("Total duration %v\n", time.Since(start))

	return nil
}

// resetBuildDirectory deletes the current build directory with everything inside and recreates an empty build folder
func resetBuildDirectory() error {
	if err := os.RemoveAll(buildPath); err != nil {
		return errors.New("could not reset build path")
	}

	if err := os.Mkdir(buildPath, os.ModePerm); err != nil {
		return errors.New("could not create build path")
	}

	return nil
}

// getArticles returns a sorted list of articles found in the /articles folder
func getArticles() (articles, error) {
	var articles articles

	files, err := ioutil.ReadDir(articlesPath)
	if err != nil {
		return articles, err
	}

	var fileList []string
	for _, file := range files {
		fileName := file.Name()

		if !file.IsDir() && filepath.Ext(fileName) == ".md" && !strings.HasPrefix(fileName, "_") {
			fileList = append(fileList, fileName)
		}
	}

	sort.Sort(sort.Reverse(sort.StringSlice(fileList)))

	for _, file := range fileList {
		article := article{MarkdownPath: file}
		article, _ = article.parse() // TODO: handle error

		articles = append(articles, article)
	}

	return articles, nil
}
