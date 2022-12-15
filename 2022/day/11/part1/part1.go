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

func go_for_rounds(blocks []string, monkeys []Monkey) {
	for round := 0; round < 20; round++ {
		for _, block := range blocks {
			groups := regexp.MustCompile(`Monkey\s(\d+):\n\s+Starting items:\s([\d,\s]+)\n\s+Operation:\snew\s=\sold\s([+*])\s(old|\d+)\n\s+Test:\sdivisible by\s(\d+)\n\s+If true: throw to monkey\s(\d+)\n\s+If false: throw to monkey (\d+)`).FindStringSubmatch(block)

			monkey_index, _ := strconv.Atoi(groups[1])
			items := parse_items(groups[2])
			operation := groups[3]
			op_value := groups[4]
			divisible, _ := strconv.Atoi(groups[5])
			if_true, _ := strconv.Atoi(groups[6])
			if_false, _ := strconv.Atoi(groups[7])

			monkey := monkeys[monkey_index]

			if round == 0 {
				monkey.items = append(items, monkey.items...)
			}

			for _, item := range monkey.items {
				monkey.inspected_count++
				var op_number int
				switch op_value {
				case "old":
					op_number = item
				default:
					op_number, _ = strconv.Atoi(op_value)
				}
				worry_level := item
				switch operation {
				case "*":
					worry_level *= op_number
				case "+":
					worry_level += op_number
				}
				worry_level /= 3 //check if need to round

				var next_monkey_index int
				if worry_level%divisible == 0 {
					next_monkey_index = if_true
				} else {
					next_monkey_index = if_false
				}

				monkeys[next_monkey_index].items = append(monkeys[next_monkey_index].items, worry_level)
			}
			monkey.items = nil

			monkeys[monkey_index] = monkey
		}

	}
}

func get_top_two(monkeys []Monkey) [2]int {
	top_two := [2]int{}
	for _, monkey := range monkeys {
		if monkey.inspected_count > top_two[0] {
			top_two[1] = top_two[0]
			top_two[0] = monkey.inspected_count
		}
	}
	return top_two
}

func count_level_of_monkey_business(top_two [2]int) int {
	return top_two[0] * top_two[1]
}

func main() {
	dat, _ := os.ReadFile("data/input.txt")

	blocks := strings.Split(string(dat), "\n\n")

	monkeys := make([]Monkey, len(blocks))

	go_for_rounds(blocks, monkeys)

	fmt.Println(monkeys)

	top_two := get_top_two(monkeys)
	fmt.Println(top_two)
	fmt.Println(count_level_of_monkey_business(top_two))

}
