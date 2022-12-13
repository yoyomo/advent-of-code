package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func map_forest(lines []string, forest [][]uint64) {
	for l, line := range lines {
		forest[l] = make([]uint64, len(line))
		for c, char := range []rune(line) {
			num, _ := strconv.ParseUint(string(char), 10, 8)
			forest[l][c] = num
		}
	}
}

func top_score(forest [][]uint64, r int, c int) uint64 {
	tree := forest[r][c]
	var score uint64 = 0
	for top := r - 1; top >= 0; top-- {
		score += 1

		if forest[top][c] >= tree {
			return score
		}
	}
	return score
}
func bottom_score(forest [][]uint64, r int, c int, rsize int) uint64 {
	tree := forest[r][c]
	var score uint64 = 0

	for bottom := r + 1; bottom <= rsize-1; bottom++ {
		score += 1

		if forest[bottom][c] >= tree {
			return score
		}
	}
	return score
}
func left_score(forest [][]uint64, r int, c int) uint64 {
	tree := forest[r][c]
	var score uint64 = 0

	for left := c - 1; left >= 0; left-- {
		score += 1
		if forest[r][left] >= tree {
			return score
		}
	}
	return score
}
func right_score(forest [][]uint64, r int, c int, csize int) uint64 {
	tree := forest[r][c]
	var score uint64 = 0

	for right := c + 1; right <= csize-1; right++ {
		score += 1
		if forest[r][right] >= tree {
			return score
		}
	}
	return score
}

func calculate_scenic_score(forest [][]uint64, r int, c int, rsize int, csize int) uint64 {
	var top, bottom, left, right uint64
	if c == 0 || c == csize-1 {
		return 0
	}
	if r == 0 || r == rsize-1 {
		return 0
	}
	top = top_score(forest, r, c)
	bottom = bottom_score(forest, r, c, rsize)
	left = left_score(forest, r, c)
	right = right_score(forest, r, c, csize)

	return top * bottom * left * right
}

func count_scenic_scores(forest [][]uint64) uint64 {
	var highest_scenic_score uint64 = 0
	for r, row := range forest {
		for c := range row {
			scenic_score := calculate_scenic_score(forest, r, c, len(forest), len(row))
			if scenic_score >= highest_scenic_score {
				highest_scenic_score = scenic_score
			}
		}
	}
	return highest_scenic_score
}

func main() {
	dat, _ := os.ReadFile("data/input.txt")

	lines := strings.Split(string(dat), "\n")
	forest := make([][]uint64, len(lines))

	map_forest(lines, forest)

	fmt.Println(count_scenic_scores(forest))

}
