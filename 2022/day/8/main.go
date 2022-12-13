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

func is_top_visible(forest [][]uint64, r int, c int) bool {
	tree := forest[r][c]
	for top := r - 1; top >= 0; top-- {
		if forest[top][c] >= tree {
			return false
		}
	}
	return true
}
func is_bottom_visible(forest [][]uint64, r int, c int, rsize int) bool {
	tree := forest[r][c]
	for bottom := r + 1; bottom <= rsize-1; bottom++ {
		if forest[bottom][c] >= tree {
			return false
		}
	}
	return true
}
func is_left_visible(forest [][]uint64, r int, c int) bool {
	tree := forest[r][c]
	for left := c - 1; left >= 0; left-- {
		if forest[r][left] >= tree {
			return false
		}
	}
	return true
}
func is_right_visible(forest [][]uint64, r int, c int, csize int) bool {
	tree := forest[r][c]
	for right := c + 1; right <= csize-1; right++ {
		if forest[r][right] >= tree {
			return false
		}
	}
	return true
}

func is_visible(forest [][]uint64, r int, c int, rsize int, csize int) bool {
	if c == 0 || c == csize-1 {
		return true
	}
	if r == 0 || r == rsize-1 {
		return true
	}
	if is_top_visible(forest, r, c) {
		return true
	}
	if is_bottom_visible(forest, r, c, rsize) {
		return true
	}
	if is_left_visible(forest, r, c) {
		return true
	}
	if is_right_visible(forest, r, c, csize) {
		return true
	}

	return false
}

func count_visible_trees(forest [][]uint64) uint64 {
	var visible_trees uint64 = 0
	for r, row := range forest {
		for c := range row {
			if is_visible(forest, r, c, len(forest), len(row)) {
				visible_trees += 1
			}
		}
	}
	return visible_trees
}

func main() {
	dat, _ := os.ReadFile("data/input.txt")

	lines := strings.Split(string(dat), "\n")
	forest := make([][]uint64, len(lines))

	map_forest(lines, forest)

	fmt.Println(count_visible_trees(forest))

}
