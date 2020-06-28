package build

import (
	"path/filepath"
	"sync"

	"github.com/fatih/color"
	"github.com/otiai10/copy"
)

func copyAssets(wg *sync.WaitGroup) {
	defer wg.Done()

	if err := copy.Copy(filepath.Join(themePath, "favicon.png"), filepath.Join(buildPath, "favicon.png")); err != nil {
		color.Red(err.Error())
		return
	}

	if err := copy.Copy(filepath.Join(themePath, "style.css"), filepath.Join(buildPath, "style.css")); err != nil {
		color.Red(err.Error())
		return
	}

	if err := copy.Copy(staticPath, buildPath); err != nil {
		color.Red(err.Error())
		return
	}

}
