package main

import (
	"strconv"
	"strings"
)

func getUnsafeIndexes(lines [][]int) [][]int {
	unsafeIndexes := make([][]int, len(lines))
	for lineIndex, numbers := range lines {
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
				unsafeIndexes[lineIndex] = append(unsafeIndexes[lineIndex], i)
				continue
			}
			if diff > 3 {
				unsafeIndexes[lineIndex] = append(unsafeIndexes[lineIndex], i)
				continue
			}
			if isAlreadyIncreasing == 0 {
				isAlreadyIncreasing = isIncreasing
			} else if isIncreasing != isAlreadyIncreasing {
				unsafeIndexes[lineIndex] = append(unsafeIndexes[lineIndex], i)
				continue
			}
		}
	}

	return unsafeIndexes
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
	intLines := stringToIntList(lines)
	unsafeIndexes := getUnsafeIndexes(intLines)

	// remove the unsafe indexes
	var newLines = make([][]int, len(intLines))
	for linesIndex, unsafeIndex := range unsafeIndexes {
		if len(unsafeIndex) != 1 {
			newLines[linesIndex] = intLines[linesIndex]
			continue
		}
		firstHalf := intLines[linesIndex][:unsafeIndex[0]]
		secondHalf := intLines[linesIndex][unsafeIndex[0]+1:]
		newLines[linesIndex] = append(firstHalf, secondHalf...)
	}

	unsafeIndexes = getUnsafeIndexes(newLines)
	safeTotal := 0
	for _, unsafeIndex := range unsafeIndexes {
		if len(unsafeIndex) == 0 {
			safeTotal++
		}
	}

	return safeTotal
}
