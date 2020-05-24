package build

import (
	"path/filepath"

	"github.com/otiai10/copy"
)

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
