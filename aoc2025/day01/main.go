package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/SanderMar/adventofcode/aoc2025/util"
)

func mod(dividend, divisor int) int {
	r := dividend % divisor
	if r < 0 {
		r += divisor
	}
	return r
}

func Abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func part1(input string) int {
	numZero := 0
	pos := 50
	for line := range strings.SplitSeq(input, "\n") {
		numStr := line[1:]
		num, err := strconv.Atoi(numStr)
		util.Check(err)
		if line[0] == 'L' {
			pos -= num
		} else {
			pos += num
		}
		pos = mod(pos, 100)
		if pos == 0 {
			numZero += 1
		}
	}

	return numZero
}

func part2(input string) int {
	numZero := 0
	pos := 50
	for line := range strings.SplitSeq(input, "\n") {
		numStr := line[1:]
		steps, err := strconv.Atoi(numStr)
		util.Check(err)

		// Amount of full rotations performed, each adds a step landing on 0
		numZero += steps / 100
		// Steps remaining after completing all full rotations
		stepsRemain := mod(steps, 100)
		if stepsRemain == 0 {
			// All steps completed
			continue
		}
		// Get direction of the steps
		if line[0] == 'L' {
			stepsRemain = -stepsRemain
		}
		newPos := pos + stepsRemain
		if ((newPos <= 0) || (newPos >= 100)) && pos != 0 {
			numZero += 1
		}
		// numZero += int(math.Abs(math.Floor(float64(newPos) / 100.0)))
		pos = mod(newPos, 100)
	}

	return numZero
}

func main() {
	input := util.GetInput("day01.txt")

	fmt.Println("Part 1:")
	res := part1(input)
	fmt.Printf("\t%d\n", res)
	fmt.Println("Part 2:")
	res = part2(input)
	fmt.Printf("\t%d\n", res)
}
