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

		print(groups[1], " ", groups[2], "\n")
		firstNumber, _ := strconv.Atoi(groups[1])
		firstList = append(firstList, firstNumber)
		secondNumber, _ := strconv.Atoi(groups[2])
		secondList = append(secondList, secondNumber)
	}

	// sort the lists
	sort.Ints(firstList)
	sort.Ints(secondList)

	// find the absolute difference between the two lists
	diff := 0
	for i := 0; i < len(firstList); i++ {
		difference := firstList[i] - secondList[i]
		if difference < 0 {
			difference = -difference
		}
		print(difference, "\n")
		diff += difference
	}
	print(diff, "\n")
}
