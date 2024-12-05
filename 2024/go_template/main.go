package main

import (
	"os"
	"regexp"
	"strings"
)

func main() {
	dat, _ := os.ReadFile("data/sample.txt")

	lines := strings.Split(string(dat), "\n")

	for _, line := range lines {
		re := regexp.MustCompile(``)
		re.FindStringSubmatch(line)
	}

	print(part1(), "\n")
	print(part2(), "\n")
}
