package main

func part2(firstList []int, secondList []int) int {
	// find how many times each number in the first list appears in the second list
	// and return the sum of the occurrences
	total := 0
	for _, firstNumber := range firstList {
		occurrences := 0
		for _, secondNumber := range secondList {
			if firstNumber == secondNumber {
				occurrences++
			}
		}
		total += firstNumber * occurrences
	}
	return total
}
