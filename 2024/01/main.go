package main

import (
	"os"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

func main() {
	dat, _ := os.ReadFile("data/input.txt")

	lines := strings.Split(string(dat), "\n")

	// build the lists
	firstList := make([]int, 0)
	secondList := make([]int, 0)
	for _, line := range lines {
		re := regexp.MustCompile(`(\d+)\s+(\d+)`)
		groups := re.FindStringSubmatch(line)

		firstNumber, _ := strconv.Atoi(groups[1])
		firstList = append(firstList, firstNumber)
		secondNumber, _ := strconv.Atoi(groups[2])
		secondList = append(secondList, secondNumber)
	}

	// sort the lists
	sort.Ints(firstList)
	sort.Ints(secondList)

	print(part1(firstList, secondList), "\n")
	print(part2(firstList, secondList), "\n")
}
