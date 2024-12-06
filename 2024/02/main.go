package main

import (
	"os"
	"strings"
)

func main() {
	dat, _ := os.ReadFile("data/input.txt")

	lines := strings.Split(string(dat), "\n")

	print(part1(lines), "\n")
	print(part2(lines), "\n")
}
