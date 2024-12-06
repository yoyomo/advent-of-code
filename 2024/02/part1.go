package main

import (
	"strconv"
	"strings"
)

func part1(lines []string) int {

	isSafeTotal := 0
REPORT:
	for _, line := range lines {
		isAlreadyIncreasing := 0
		numbers := strings.Split(line, " ")
		for i := 0; i < len(numbers)-1; i++ {
			number, _ := strconv.Atoi(numbers[i])
			nextNumber, _ := strconv.Atoi(numbers[i+1])
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
