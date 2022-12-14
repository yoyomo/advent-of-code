package main

import (
	"fmt"
	"math"
	"os"
	"regexp"
	"strconv"
	"strings"
)

var NUM_OF_POSITIONS uint64 = 10

type Table struct {
	row int64
	col int64
}

func get_position_key(positions []Table) string {
	row := positions[NUM_OF_POSITIONS-1].row
	col := positions[NUM_OF_POSITIONS-1].col
	return strconv.FormatInt(row, 10) + "," + strconv.FormatInt(col, 10)
}

func is_out_of_range(positions []Table, head uint64, tail uint64) bool {

	return math.Abs(float64(positions[head].col-positions[tail].col)) >= 2 || math.Abs(float64(positions[head].row-positions[tail].row)) >= 2
}

func update_positions(positions []Table, head uint64, direction string) {
	var row, col int64
	switch direction {
	case "R":
		row, col = 0, 1
	case "L":
		row, col = 0, -1
	case "U":
		row, col = -1, 0
	case "D":
		row, col = 1, 0
	}

	tail := head + 1

	if head == 0 {
		positions[head].row += row
		positions[head].col += col
	}
	if is_out_of_range(positions, head, tail) {

		if positions[head].row > positions[tail].row {
			positions[tail].row++
		} else if positions[head].row < positions[tail].row {
			positions[tail].row--
		}
		if positions[head].col > positions[tail].col {
			positions[tail].col++
		} else if positions[head].col < positions[tail].col {
			positions[tail].col--
		}
	}
}

func main() {
	dat, _ := os.ReadFile("data/input.txt")

	lines := strings.Split(string(dat), "\n")

	routes := make(map[string]uint64)

	positions := make([]Table, NUM_OF_POSITIONS)
	routes[get_position_key(positions)] = 1
	for _, line := range lines {
		re := regexp.MustCompile(`(\w) (\d+)`)
		groups := re.FindStringSubmatch(line)
		direction := groups[1]
		times, _ := strconv.ParseUint(groups[2], 10, 64)

		for i := uint64(0); i < times; i++ {
			for j := uint64(0); j < NUM_OF_POSITIONS-1; j++ {
				update_positions(positions, j, direction)
			}

			routes[get_position_key(positions)]++
		}

	}

	fmt.Println(len(routes))

}
