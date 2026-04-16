package main

import (
	"fmt"
	"math"
	"strings"

	"github.com/SanderMar/adventofcode/aoc2025/util"
)

func parseInput(input string) [][]int {
	split := strings.Split(input, "\n")
	ints := make([][]int, len(split))
	for i, line := range split {
		lineInts := make([]int, len(line))

		for i, r := range line {
			lineInts[i] = int(r - '0')
		}

		ints[i] = lineInts
	}
	return ints
}

func findVoltage(arr []int, left int) int {
	if left == 0 {
		return 0
	}
	left--
	hIdx := 0
	for i := 1; i < len(arr)-left; i++ {
		if arr[i] > arr[hIdx] {
			hIdx = i
		}
	}
	foundLower := findVoltage(arr[hIdx+1:], left)
	return arr[hIdx]*int(math.Pow10(left)) + foundLower
}

func part1(input [][]int) int {
	sum := 0
	for _, bank := range input {
		sum += findVoltage(bank, 2)
	}
	return sum
}

func part2(input [][]int) int {
	sum := 0
	for _, bank := range input {
		sum += findVoltage(bank, 12)
	}
	return sum
}

func main() {
	input := parseInput(util.GetInput("day03.txt"))

	fmt.Println("Part 1:")
	res := part1(input)
	fmt.Printf("\t%d\n", res)
	fmt.Println("Part 2:")
	res = part2(input)
	fmt.Printf("\t%d\n", res)
}
