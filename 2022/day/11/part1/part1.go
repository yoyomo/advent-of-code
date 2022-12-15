package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

type Monkey struct {
	items           []int
	inspected_count int
}

func parse_items(str string) []int {
	str_items := strings.Split(string(str), ", ")
	items := []int{}
	for _, str_item := range str_items {
		item, _ := strconv.Atoi(str_item)
		items = append(items, item)
	}
	return items
}

func main() {
	dat, _ := os.ReadFile("data/input.txt")

	blocks := strings.Split(string(dat), "\n\n")

	round := 0
	monkeys := make(map[int]Monkey)
	for _, block := range blocks {
		groups := regexp.MustCompile(`Monkey\s(\d+):\n\s+Starting items:\s([\d,\s]+)\n\s+Operation:\snew\s=\sold\s([+*])\s(old|\d+)\n\s+Test:\sdivisible by\s(\d+)\n\s+If true: throw to monkey\s(\d+)\n\s+If false: throw to monkey (\d+)`).FindStringSubmatch(block)

		monkey_index, _ := strconv.Atoi(groups[1])
		items := parse_items(groups[2])
		operation := groups[3]
		op_number, _ := strconv.Atoi(groups[4])
		divisible, _ := strconv.Atoi(groups[5])
		if_true, _ := strconv.Atoi(groups[6])
		if_false, _ := strconv.Atoi(groups[7])

		monkey := monkeys[monkey_index]

		monkey.items = append(items, monkey.items...)

		if monkey_index == 0 {
			round++
		}

		for _, item := range monkey.items {
			monkey.inspected_count++
			worry_level := item
			switch operation {
			case "*":
				worry_level *= op_number
			case "+":
				worry_level += op_number
			}
			worry_level /= 3 //check if need to round

			var next_monkey Monkey
			if worry_level%divisible == 0 {
				next_monkey = monkeys[if_true]

			} else {
				next_monkey = monkeys[if_false]

			}
			next_monkey.items = append(next_monkey.items, worry_level)
		}

		if round == 20 {
			break
		}

	}

	fmt.Println(monkeys)

}
