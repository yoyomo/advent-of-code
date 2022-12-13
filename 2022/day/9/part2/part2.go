package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func get_position_key(positions []Table) string {
	row := positions[NUM_OF_POSITIONS-1].row
	col := positions[NUM_OF_POSITIONS-1].col
	return strconv.FormatInt(row, 10) + "," + strconv.FormatInt(col, 10)
}

var NUM_OF_POSITIONS uint64 = 10

type Table struct {
	row int64
	col int64
}

func update_position_right(positions []Table, head uint64, tail uint64) {
	positions[head].col++
	if positions[head].col-positions[tail].col >= 2 {
		positions[tail].col++
		if positions[head].row != positions[tail].row {
			positions[tail].row = positions[head].row
		}
	}
}

func update_position_left(positions []Table, head uint64, tail uint64) {
	positions[head].col--
	if positions[head].col-positions[tail].col <= -2 {
		positions[tail].col--
		if positions[head].row != positions[tail].row {
			positions[tail].row = positions[head].row
		}
	}
}

func update_position_up(positions []Table, head uint64, tail uint64) {
	positions[head].row--
	if positions[head].row-positions[tail].row <= -2 {
		positions[tail].row--
		if positions[head].col != positions[tail].col {
			positions[tail].col = positions[head].col
		}
	}
}

func update_position_down(positions []Table, head uint64, tail uint64) {
	positions[head].row++
	if positions[head].row-positions[tail].row >= 2 {
		positions[tail].row++
		if positions[head].col != positions[tail].col {
			positions[tail].col = positions[head].col
		}
	}
}

func main() {
	dat, _ := os.ReadFile("data/example2.txt")

	lines := strings.Split(string(dat), "\n")

	routes := make(map[string]uint64)

	positions := make([]Table, NUM_OF_POSITIONS)
	routes[get_position_key(positions)] = 1
	for _, line := range lines {
		fmt.Println(line)
		re := regexp.MustCompile(`(\w) (\d+)`)
		groups := re.FindStringSubmatch(line)
		direction := groups[1]
		times, _ := strconv.ParseUint(groups[2], 10, 64)

		for i := uint64(0); i < times; i++ {
			for j := uint64(0); j < NUM_OF_POSITIONS-1; j += 2 {
				fmt.Printf("j: %v\n", j)
				switch direction {
				case "R":
					update_position_right(positions, j, j+1)
				case "L":
					update_position_left(positions, j, j+1)
				case "U":
					update_position_up(positions, j, j+1)
				case "D":
					update_position_down(positions, j, j+1)
				}
			}
			routes[get_position_key(positions)]++
		}
	}

	fmt.Println(NUM_OF_POSITIONS - 1)
	fmt.Println(positions)

	fmt.Println(routes)

	places := 0
	for _, count := range routes {
		if count >= 1 {
			places++
		}
	}

	fmt.Println(places)

}
