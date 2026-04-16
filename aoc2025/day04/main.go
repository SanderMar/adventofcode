package main

import (
	"fmt"
	"strings"

	"github.com/SanderMar/adventofcode/aoc2025/util"
)

func parseInput(input string) ([]int, int) {
	split := strings.Split(input, "\n")
	rowLen := len(split[0])
	grid := make([]int, rowLen*len(split))
	for i, row := range split {
		for j, r := range row {
			val := 0
			if r == '@' {
				val = 1
			}
			grid[j+i*rowLen] = val
		}
	}
	return grid, rowLen
}

func genNeighbors(cur, rowLen, gridLen int) []int {
	dirs := make([]int, 0, 8)
	curCol := cur % rowLen
	for dx := -1; dx <= 1; dx++ {
		dirRow := cur + dx*rowLen
		if dirRow < 0 || dirRow >= gridLen {
			continue
		}
		for dy := -1; dy <= 1; dy++ {
			if dx == 0 && dy == 0 {
				// Don't include self
				continue
			}
			if dirCol := curCol + dy; dirCol < 0 || dirCol >= rowLen {
				continue
			}
			dirs = append(dirs, dirRow+dy)
		}
	}
	return dirs
}

func canRemove(grid []int, rowLen int) []int {
	toRemove := make([]int, 0, len(grid))
	for i, v := range grid {
		if v == 0 {
			continue
		}
		neighbors := 0
		for _, nIdx := range genNeighbors(i, rowLen, len(grid)) {
			neighbors += grid[nIdx]
		}
		if neighbors < 4 {
			toRemove = append(toRemove, i)
		}
	}
	return toRemove
}

func part1(input string) int {
	return len(canRemove(parseInput(input)))
}

func part2(input string) int {
	grid, rowLen := parseInput(input)
	sum := 0
	for {
		toRemove := canRemove(grid, rowLen)
		if len(toRemove) == 0 {
			break
		}
		sum += len(toRemove)
		for _, idx := range toRemove {
			grid[idx] = 0
		}
	}
	return sum
}

func main() {
	input := util.GetInput("day04.txt")

	fmt.Println("Part 1:")
	res := part1(input)
	fmt.Printf("\t%d\n", res)
	fmt.Println("Part 2:")
	res = part2(input)
	fmt.Printf("\t%d\n", res)
}
