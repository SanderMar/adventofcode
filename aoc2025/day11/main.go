package main

import (
	"fmt"
	"maps"
	"strings"

	"github.com/SanderMar/adventofcode/aoc2025/util"
)

type device struct {
	name    string
	outputs []*device
}

func (d device) outPaths(mustPass map[string]bool) int {
	mustPass = maps.Clone(mustPass) // Clone to not update map outside of scope

	if d.name == "out" {
		// Reached end
		passedAll := true
		for _, passed := range mustPass {
			passedAll = passedAll && passed
		}
		if passedAll {
			return 1
		}
		return 0
	} else if len(d.outputs) == 0 {
		// No further connections, doesn't reach end
		return 0
	} else if len(d.outputs) == 1 && d.outputs[0].name == d.name {
		// Connected to self for some weird reason, doesn't reach end
		return 0
	}
	// Check if we must pass the current device, mark as passed if so
	_, present := mustPass[d.name]
	if present {
		mustPass[d.name] = true
	}
	sum := 0
	for _, connected := range d.outputs {
		sum += connected.outPaths(mustPass)
	}
	return sum
}

func parseInput(input string) map[string]*device {
	lines := strings.Split(input, "\n")

	deviceMap := make(map[string]*device)
	devices := make([]*device, 0, len(lines))

	deviceMap["out"] = &device{"out", []*device{}}
	for _, l := range lines {
		splitted := strings.Split(l, ":")
		d := device{splitted[0], []*device{}}
		deviceMap[splitted[0]] = &d
		devices = append(devices, &d)
	}
	for i, l := range lines {
		splitted := strings.Split(l, " ")
		device := devices[i]
		for _, name := range splitted[1:] {
			device.outputs = append(device.outputs, deviceMap[name])
		}
	}

	return deviceMap
}

func part1(input string) int {
	devices := parseInput(input)
	return devices["you"].outPaths(map[string]bool{})
}

func part2(input string) int {
	devices := parseInput(input)
	passMap := map[string]bool{"dac": false, "fft": false}
	return devices["svr"].outPaths(passMap)
}

func main() {
	input := util.GetInput("day11.txt")

	fmt.Println("Part 1:")
	res := part1(input)
	fmt.Printf("\t%d\n", res)
	fmt.Println("Part 2:")
	res = part2(input)
	fmt.Printf("\t%d\n", res)
}
