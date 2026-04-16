package main

import (
	"cmp"
	"fmt"
	"slices"
	"strconv"
	"strings"

	"github.com/SanderMar/adventofcode/aoc2025/util"
)

func parseInput(input string) ([][2]int, []int) {
	splitted := strings.Split(input, "\n\n")
	ranges := parseRanges(splitted[0])
	ids := parseIDs(splitted[1])
	return ranges, ids
}

func parseIDs(input string) []int {
	idStrings := strings.Split(input, "\n")
	ids := make([]int, len(idStrings))
	for i, idStr := range idStrings {
		id, err := strconv.Atoi(idStr)
		util.Check(err)
		ids[i] = id
	}
	slices.Sort(ids)
	return ids
}

func parseRanges(input string) [][2]int {
	var ranges [][2]int
	for rangeLine := range strings.SplitSeq(input, "\n") {
		// Split line into lower and upper values
		rangeStr := strings.Split(rangeLine, "-")
		// Extract lower and upper values as numbers
		lower, err := strconv.Atoi(rangeStr[0])
		util.Check(err)
		upper, err := strconv.Atoi(rangeStr[1])
		util.Check(err)
		ranges = append(ranges, [2]int{lower, upper})
	}
	// Sort ranges to go over them in ascending order
	slices.SortFunc(ranges, func(a, b [2]int) int { return cmp.Compare(a[0], b[0]) })
	// Collapse ranges so that no overlap exists in returned ranges
	var collapsed [][2]int
	for i, r := range ranges {
		colL := len(collapsed) - 1
		if i > 0 {
			if collapsed[colL][1] >= r[0] && collapsed[colL][1] < r[1] {
				// Extend previous range
				collapsed[colL][1] = r[1]
				continue
			} else if collapsed[colL][0] <= r[0] && r[1] <= collapsed[colL][1] {
				// Fits completely in current range
				continue
			}
		}
		// Non-overlapping range
		collapsed = append(collapsed, r)
	}
	return collapsed
}

func part1(input string) int {
	// Sorted ranges (collapsed) and ids
	ranges, ids := parseInput(input)
	rIdx := 0
	sum := 0
	// Go over sorted ids
	for _, id := range ids {
		if id < ranges[rIdx][0] {
			continue
		}
		// Id exceeds the current range lower bound. Look for new range
		// As ranges are sorted in ascending order, we can just go over them
		for _, r := range ranges[rIdx:] {
			if id < r[0] {
				// Id falls bellow current range under consideration
				// Update current range and move on to next idea
				break
			} else if r[0] <= id && id <= r[1] {
				sum += 1
				break
			}
			rIdx++
		}
	}
	return sum
}

func part2(input string) int {
	ranges, _ := parseInput(input)

	sum := 0
	for _, r := range ranges {
		sum += r[1] - r[0] + 1
	}

	return sum
}

func main() {
	input := util.GetInput("day05.txt")

	fmt.Println("Part 1:")
	res := part1(input)
	fmt.Printf("\t%d\n", res)
	fmt.Println("Part 2:")
	res = part2(input)
	fmt.Printf("\t%d\n", res)
}
