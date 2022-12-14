package main

import (
	"os"
	"regexp"
	"strings"
)

func main() {
	dat, _ := os.ReadFile("data/input.txt")

	lines := strings.Split(string(dat), "\n")

	for _, line := range lines {
		re := regexp.MustCompile(``)
		re.FindStringSubmatch(line)
	}

}
