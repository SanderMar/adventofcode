package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/SanderMar/adventofcode/aoc2025/util"
)

func getBounds(in string) (int, int) {
	vals := strings.Split(in, "-")
	start, err := strconv.Atoi(vals[0])
	util.Check(err)
	end, err := strconv.Atoi(vals[1])
	util.Check(err)
	return start, end
}

func part1(input string) int {
	sum := 0
	for r := range strings.SplitSeq(input, ",") {
		start, end := getBounds(r)
		for cur := start; cur <= end; cur++ {
			curStr := strconv.Itoa(cur)
			strLen := len(curStr)

			if strLen%2 == 1 {
				continue
			}

			if curStr[:strLen/2] == curStr[strLen/2:] {
				sum += cur
			}
		}
	}
	return sum
}

func isInvalid2(input int) bool {
	inStr := strconv.Itoa(input)
	inLen := len(inStr)
	halfLen := inLen / 2
	for i := 1; i <= halfLen; i++ {
		if inLen%i == 1 {
			continue
		}
		seq := inStr[:i]
		if strings.Repeat(seq, inLen/i) == inStr {
			return true
		}
	}
	return false
}

func part2(input string) int {
	sum := 0
	for r := range strings.SplitSeq(input, ",") {
		start, end := getBounds(r)
		for cur := start; cur <= end; cur++ {
			if isInvalid2(cur) {
				sum += cur
			}
		}
	}
	return sum
}

func main() {
	input := util.GetInput("day02.txt")
	// input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"

	fmt.Println("Part 1:")
	res := part1(input)
	fmt.Printf("\t%d\n", res)
	fmt.Println("Part 2:")
	res = part2(input)
	fmt.Printf("\t%d\n", res)
}
