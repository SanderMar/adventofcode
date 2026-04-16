package util

import (
	"os"
	"path/filepath"
	"strings"
)

func Check(e error) {
	if e != nil {
		panic(e)
	}
}

func GetInput(filename string) string {
	path := filepath.Join("input", filename)
	dat, err := os.ReadFile(path)
	Check(err)
	return strings.TrimRight(string(dat), "\n")
}
