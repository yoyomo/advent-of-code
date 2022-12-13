package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func get_position(row int64, col int64) string {
	return strconv.FormatInt(row, 10) + "-" + strconv.FormatInt(col, 10)
}

func main() {
	dat, _ := os.ReadFile("data/input.txt")

	lines := strings.Split(string(dat), "\n")

	routes := make(map[string]uint64)
	var hrow, hcol int64 = 0, 0
	var trow, tcol int64 = 0, 0
	routes[get_position(trow, tcol)] = 1
	for _, line := range lines {
		fmt.Println(line)
		re := regexp.MustCompile(`(\w) (\d+)`)
		groups := re.FindStringSubmatch(line)
		direction := groups[1]
		times, _ := strconv.ParseUint(groups[2], 10, 64)

		for i := uint64(0); i < times; i++ {
			switch direction {
			case "R":
				hcol++
				if hcol-tcol >= 2 {
					tcol++
					if hrow != trow {
						trow = hrow
					}
				}
			case "L":
				hcol--
				if hcol-tcol <= -2 {
					tcol--
					if hrow != trow {
						trow = hrow
					}
				}
			case "U":
				hrow--
				if hrow-trow <= -2 {
					trow--
					if hcol != tcol {
						tcol = hcol
					}
				}
			case "D":
				hrow++
				if hrow-trow >= 2 {
					trow++
					if hcol != tcol {
						tcol = hcol
					}
				}
			}

			routes[get_position(trow, tcol)]++

		}
	}

	fmt.Println(routes)

	places := 0
	for _, count := range routes {
		if count >= 1 {
			places++
		}
	}

	fmt.Println(places)

}
