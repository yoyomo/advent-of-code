package main

func part1(firstList []int, secondList []int) int {
	// find the absolute difference between the two lists
	diff := 0
	for i := 0; i < len(firstList); i++ {
		difference := firstList[i] - secondList[i]
		if difference < 0 {
			difference = -difference
		}
		diff += difference
	}
	return diff
}
