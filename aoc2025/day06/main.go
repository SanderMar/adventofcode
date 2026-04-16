package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/SanderMar/adventofcode/aoc2025/util"
)

type problem struct {
	oper rune
	s    int
	nums []int
}

func (p problem) solve() int {
	if p.oper != '*' && p.oper != '+' {
		panic(fmt.Errorf("Unknown operator: %v", p.oper))
	}
	res := p.nums[0]
	for _, val := range p.nums[1:] {
		switch p.oper {
		case '*':
			res *= val
		case '+':
			res += val
		}
	}
	return res
}

func parseOperLine(line string) []problem {
	var problems []problem
	var curProblem problem
	for i, r := range line {
		lastProb := len(problems) - 1
		if r != ' ' {
			curProblem = problem{r, i, []int{0}}
			problems = append(problems, curProblem)
			if lastProb >= 0 {
				// Remove buffer space between problems
				lastProbLen := len(problems[lastProb].nums)
				problems[lastProb].nums = problems[lastProb].nums[:lastProbLen-1]
			}
		} else {
			// Extra column in problem
			problems[lastProb].nums = append(problems[lastProb].nums, 0)
		}
	}
	return problems
}

func parseNumLine1(line string, problems []problem) []problem {
	vals := strings.Fields(line)
	for i, v := range vals {
		vAsInt, err := strconv.Atoi(v)
		util.Check(err)
		problems[i].nums = append(problems[i].nums, vAsInt)
	}
	return problems
}

func parseNumLine2(line string, problems []problem) []problem {
	for i, p := range problems {
		for j, r := range line[p.s : p.s+len(p.nums)] {
			if r == ' ' {
				continue
			}
			p.nums[j] = p.nums[j]*10 + int(r-'0')
		}
		problems[i] = p
	}
	return problems
}

func part1(input string) int {
	lines := strings.Split(input, "\n")
	linesCnt := len(lines)
	problems := parseOperLine(lines[linesCnt-1])
	for i := range problems {
		problems[i].nums = make([]int, 0, linesCnt-1)
	}
	for _, line := range lines[:linesCnt-1] {
		problems = parseNumLine1(line, problems)
	}
	sum := 0
	for _, p := range problems {
		sum += p.solve()
	}
	return sum
}

func part2(input string) int {
	lines := strings.Split(input, "\n")
	linesCnt := len(lines)
	problems := parseOperLine(lines[linesCnt-1])
	for _, line := range lines[:linesCnt-1] {
		problems = parseNumLine2(line, problems)
	}
	sum := 0
	for _, p := range problems {
		sum += p.solve()
	}
	return sum
}

func main() {
	input := util.GetInput("day06.txt")

	fmt.Println("Part 1:")
	res := part1(input)
	fmt.Printf("\t%d\n", res)
	fmt.Println("Part 2:")
	res = part2(input)
	fmt.Printf("\t%d\n", res)
}
