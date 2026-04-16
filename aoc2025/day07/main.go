package main

import (
	"fmt"
	"strings"

	"github.com/SanderMar/adventofcode/aoc2025/util"
)

type stack [][2]int

func (s stack) Empty() bool {
	return len(s) == 0
}

func (s stack) Push(v [2]int) stack {
	return append(s, v)
}

func (s stack) Pop() (stack, [2]int) {
	l := len(s)
	return s[:l-1], s[l-1]
}

func parseInput(input string) ([][]int, int) {
	lines := strings.Split(input, "\n")
	grid := make([][]int, 0, len(lines))

	splits := 0
	for i, l := range lines {
		row := make([]int, len(l))
		for ri, r := range l {
			// On the first line just look for the start position
			if i == 0 {
				if r == 'S' {
					row[ri] = 1
					break
				}
				continue
			}
			// Count how many timelines pass through each cell in the grid
			above := grid[i-1][ri]
			if r == '^' {
				row[ri-1] += above
				row[ri+1] += above
				if above > 0 {
					splits++
				}
			} else {
				row[ri] += above
			}
		}
		grid = append(grid, row)
	}
	return grid, splits
}

func part1(input string) int {
	_, splits := parseInput(input)
	return splits
}

func part2(input string) int {
	timelines := 0
	grid, _ := parseInput(input)
	// Count the amount of timelines that reach the end
	for _, v := range grid[len(grid)-1] {
		timelines += v
	}
	return timelines
}

func main() {
	input := util.GetInput("day07.txt")

	fmt.Println("Part 1:")
	res := part1(input)
	fmt.Printf("\t%d\n", res)
	fmt.Println("Part 2:")
	res = part2(input)
	fmt.Printf("\t%d\n", res)
}
