package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/SanderMar/adventofcode/aoc2025/util"
)

type point [2]int

func parseInput(input string) []point {
	lines := strings.Split(input, "\n")

	points := make([]point, 0, len(lines))
	for _, l := range lines {
		numbers := strings.Split(l, ",")
		if len(numbers) != 2 {
			panic(fmt.Errorf("Wrong numbers: %v", l))
		}
		x, err := strconv.Atoi(numbers[0])
		util.Check(err)
		y, err := strconv.Atoi(numbers[1])
		points = append(points, point{x, y})
	}
	return points
}

func part1(input string) int {
	points := parseInput(input)
	maxArea := 0
	for i, p1 := range points {
		for _, p2 := range points[:i] {
			area := (max(p1[0], p2[0]) - min(p1[0], p2[0]) + 1) *
				(max(p1[1], p2[1]) - min(p1[1], p2[1]) + 1)
			maxArea = max(maxArea, area)
		}
	}
	return maxArea
}

func part2(input string) int {
	sum := 0
	return sum
}

func main() {
	input := util.GetInput("day09.txt")

	fmt.Println("Part 1:")
	res := part1(input)
	fmt.Printf("\t%d\n", res)
	fmt.Println("Part 2:")
	res = part2(input)
	fmt.Printf("\t%d\n", res)
}
