package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

type Monkey struct {
	items           []uint64
	operation       string
	op_value        string
	divisible       uint64
	if_true         uint64
	if_false        uint64
	inspected_count uint64
}

func parse_items(str string) []uint64 {
	str_items := strings.Split(string(str), ", ")
	items := []uint64{}
	for _, str_item := range str_items {
		item, _ := strconv.ParseUint(str_item, 10, 64)
		items = append(items, item)
	}
	return items
}

func init_monkeys(blocks []string, monkeys []Monkey) {
	for _, block := range blocks {
		groups := regexp.MustCompile(`Monkey\s(\d+):\n\s+Starting items:\s([\d,\s]+)\n\s+Operation:\snew\s=\sold\s([+*])\s(old|\d+)\n\s+Test:\sdivisible by\s(\d+)\n\s+If true: throw to monkey\s(\d+)\n\s+If false: throw to monkey (\d+)`).FindStringSubmatch(block)

		monkey_index, _ := strconv.ParseUint(groups[1], 10, 64)
		monkeys[monkey_index].items = append(parse_items(groups[2]), monkeys[monkey_index].items...)
		monkeys[monkey_index].operation = groups[3]
		monkeys[monkey_index].op_value = groups[4]
		monkeys[monkey_index].divisible, _ = strconv.ParseUint(groups[5], 10, 64)
		monkeys[monkey_index].if_true, _ = strconv.ParseUint(groups[6], 10, 64)
		monkeys[monkey_index].if_false, _ = strconv.ParseUint(groups[7], 10, 64)
	}
}

func calculate_lowest_common_denominator(monkeys []Monkey) uint64 {
	var lowest_common_denominator uint64
	multiples := make([]map[uint64]bool, len(monkeys))
	for i := uint64(1); lowest_common_denominator == 0; i++ {
		for m, monkey := range monkeys {
			denominator := i * monkey.divisible
			if len(multiples[m]) == 0 {
				multiples[m] = make(map[uint64]bool)
			}
			multiples[m][denominator] = true

			found := 0
			for _, multiple := range multiples {
				if multiple[denominator] {
					found++
				}
			}

			if found == len(monkeys) {
				lowest_common_denominator = denominator
				break
			}
		}
	}
	return lowest_common_denominator
}

func go_for_rounds(monkeys []Monkey, part_one bool) {
	var MAX_ROUNDS uint64
	var lowest_common_denominator uint64
	if part_one {
		MAX_ROUNDS = 20
	} else {
		MAX_ROUNDS = 10000
		lowest_common_denominator = calculate_lowest_common_denominator(monkeys)
		fmt.Println(lowest_common_denominator)
	}
	for round := uint64(0); round < MAX_ROUNDS; round++ {
		for monkey_index, monkey := range monkeys {

			for _, item := range monkey.items {
				monkey.inspected_count++
				var op_number uint64
				switch monkey.op_value {
				case "old":
					op_number = item
				default:
					op_number, _ = strconv.ParseUint(monkey.op_value, 10, 64)
				}
				worry_level := item
				switch monkey.operation {
				case "*":
					worry_level *= op_number
				case "+":
					worry_level += op_number
				}
				if part_one {
					worry_level /= 3
				} else {
					worry_level %= lowest_common_denominator
				}

				var next_monkey_index uint64
				if worry_level%monkey.divisible == 0 {
					next_monkey_index = monkey.if_true
				} else {
					next_monkey_index = monkey.if_false
				}

				monkeys[next_monkey_index].items = append(monkeys[next_monkey_index].items, worry_level)
			}
			monkey.items = nil

			monkeys[monkey_index] = monkey
		}

	}
}

func get_top_two(monkeys []Monkey) [2]uint64 {
	top_two := [2]uint64{}
	for _, monkey := range monkeys {
		if monkey.inspected_count > top_two[0] {
			top_two[1] = top_two[0]
			top_two[0] = monkey.inspected_count
		} else if monkey.inspected_count > top_two[1] {
			top_two[1] = monkey.inspected_count
		}
	}
	return top_two
}

func count_level_of_monkey_business(top_two [2]uint64) uint64 {
	return top_two[0] * top_two[1]
}

func part(blocks []string, part_one bool) {
	monkeys := make([]Monkey, len(blocks))

	init_monkeys(blocks, monkeys)

	go_for_rounds(monkeys, part_one)

	fmt.Println(monkeys)

	top_two := get_top_two(monkeys)
	fmt.Println(top_two)
	fmt.Println(count_level_of_monkey_business(top_two))
}

func main() {
	dat, _ := os.ReadFile("data/input.txt")

	blocks := strings.Split(string(dat), "\n\n")

	part(blocks, true)
	part(blocks, false)

}
