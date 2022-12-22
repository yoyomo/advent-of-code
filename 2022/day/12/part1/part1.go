package main

import (
	"fmt"
	"math"
	"os"
	"strings"

	"golang.org/x/exp/slices"
)

type Position struct {
	row int
	col int
}

func init_heightmap(rows []string, heightmap [][]rune, start_pos *Position, end_pos *Position) {
	for r, row := range rows {
		heightmap[r] = make([]rune, len(row))
		for c, col := range row {
			height := col
			if col == 'S' {
				*start_pos = Position{row: r, col: c}
				height = 'a'
			} else if col == 'E' {
				*end_pos = Position{row: r, col: c}
				height = 'z'
			}
			heightmap[r][c] = height
		}
	}
}

func get_number_of_steps(heightmap [][]rune, end_pos Position, prev_height rune, current_pos Position, list_of_steps map[string][]Position, step string) []Position {

	if current_pos.row < 0 || current_pos.row >= len(heightmap) || current_pos.col < 0 || current_pos.col >= len(heightmap[current_pos.row]) {
		return []Position{}
	}

	current_height := heightmap[current_pos.row][current_pos.col]

	if current_height > prev_height+1 || current_height < prev_height {
		return []Position{}
	}

	if slices.Contains(list_of_steps[step], current_pos) {
		return []Position{}
	}

	new_step := list_of_steps[step]
	step += string(current_height)
	list_of_steps[step] = append(new_step, current_pos)

	if current_pos == end_pos {
		return list_of_steps[step]
	}

	get_number_of_steps(heightmap, end_pos, current_height, Position{current_pos.row - 1, current_pos.col}, list_of_steps, step)
	get_number_of_steps(heightmap, end_pos, current_height, Position{current_pos.row + 1, current_pos.col}, list_of_steps, step)
	get_number_of_steps(heightmap, end_pos, current_height, Position{current_pos.row, current_pos.col - 1}, list_of_steps, step)
	get_number_of_steps(heightmap, end_pos, current_height, Position{current_pos.row, current_pos.col + 1}, list_of_steps, step)

	return []Position{}
}

func calculate_least_number_of_steps(list_of_steps map[string][]Position, end_pos Position) int {

	least_number_of_steps := math.MaxInt
	for _, step := range list_of_steps {
		if slices.Contains(step, end_pos) {
			number_of_steps := len(step) - 1
			if number_of_steps < least_number_of_steps {
				least_number_of_steps = number_of_steps
			}
		}
	}

	return least_number_of_steps
}

func main() {
	dat, _ := os.ReadFile("data/example.txt")

	rows := strings.Split(string(dat), "\n")

	heightmap := make([][]rune, len(rows))

	var start_pos, end_pos Position

	init_heightmap(rows, heightmap, &start_pos, &end_pos)

	fmt.Println(start_pos)
	fmt.Println(end_pos)

	list_of_steps := make(map[string][]Position)

	get_number_of_steps(heightmap, end_pos, 'a', start_pos, list_of_steps, "")
	fmt.Println("got it")
	fmt.Println(calculate_least_number_of_steps(list_of_steps, end_pos))

}
