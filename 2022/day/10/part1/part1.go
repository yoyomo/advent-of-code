package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func main() {
	dat, _ := os.ReadFile("data/input.txt")

	lines := strings.Split(string(dat), "\n")

	var cycle int64 = 0
	var x int64 = 1
	var next_cycle int64 = 20
	var sum int64 = 0
	for _, line := range lines {
		re := regexp.MustCompile(`(noop|addx)(?:\s([0-9\-]+))?`)
		groups := re.FindStringSubmatch(line)
		op := groups[1]
		value, _ := strconv.ParseInt(groups[2], 10, 64)

		var num_of_cycles uint8
		switch op {
		case "noop":
			num_of_cycles = 1
		case "addx":
			num_of_cycles = 2
		}

		for i := uint8(0); i < num_of_cycles; i++ {
			cycle++
			if cycle == next_cycle {
				sum += x * cycle
				next_cycle += 40
			}
		}

		switch op {
		case "noop":
		case "addx":
			x += value
		}
	}

	fmt.Println(sum)

}
