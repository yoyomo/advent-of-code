package main

import (
	"strconv"
	"strings"
)

func getSafeTotal(lines [][]int) int {
	isSafeTotal := 0
REPORT:
	for _, numbers := range lines {
		isAlreadyIncreasing := 0
		for i := 0; i < len(numbers)-1; i++ {
			number := numbers[i]
			nextNumber := numbers[i+1]
			diff := nextNumber - number
			isIncreasing := 0
			if diff < 0 {
				isIncreasing = -1
				diff = -diff
			} else if diff > 0 {
				isIncreasing = 1
			} else {
				continue REPORT
			}
			if diff > 3 {
				continue REPORT
			}
			if isAlreadyIncreasing == 0 {
				isAlreadyIncreasing = isIncreasing
			} else if isIncreasing != isAlreadyIncreasing {
				continue REPORT
			}
		}
		isSafeTotal++
	}
	return isSafeTotal
}

func stringToIntList(lines []string) [][]int {
	var numberLists [][]int
	for _, line := range lines {
		lineNumbers := strings.Split(line, " ")
		var numbers []int
		for _, lineNumber := range lineNumbers {
			number, _ := strconv.Atoi(lineNumber)
			numbers = append(numbers, number)
		}
		numberLists = append(numberLists, numbers)
	}
	return numberLists
}

func part2(lines []string) int {
	return getSafeTotal(stringToIntList(lines))
}
