package main

import (
	"cmp"
	"fmt"
	"math"
	"slices"
	"strconv"
	"strings"

	"github.com/SanderMar/adventofcode/aoc2025/util"
)

type Coord struct {
	x float64
	y float64
	z float64
}

type CoordPair struct {
	c1   Coord
	c2   Coord
	dist float64
}

type set map[*[]Coord]bool

func createCoord(coordStr string) Coord {
	vals := strings.Split(coordStr, ",")
	if len(vals) < 3 {
		panic(fmt.Errorf("Coord string %v does not contain enough coordinates", coordStr))
	}
	valsInt := make([]float64, 3)
	for i, v := range vals {
		vInt, err := strconv.Atoi(v)
		util.Check(err)
		valsInt[i] = float64(vInt)
	}
	return Coord{valsInt[0], valsInt[1], valsInt[2]}
}

func (t Coord) dist(o Coord) float64 {
	return math.Sqrt(math.Pow(t.x-o.x, 2) + math.Pow(t.y-o.y, 2) + math.Pow(t.z-o.z, 2))
}

func parseInput(input string) ([]CoordPair, []Coord) {
	splitted := strings.Split(input, "\n")
	coords := make([]Coord, 0, len(splitted))
	var distances []CoordPair
	for _, line := range splitted {
		curCoord := createCoord(line)
		for _, coord := range coords {
			distances = append(distances, CoordPair{curCoord, coord, curCoord.dist(coord)})
		}
		coords = append(coords, curCoord)
	}
	return distances, coords
}

func MergeCircuits(p CoordPair, circs map[Coord]*[]Coord) map[Coord]*[]Coord {
	for _, c := range *circs[p.c2] {
		*circs[p.c1] = append(*circs[p.c1], c)
		circs[c] = circs[p.c1]
	}
	return circs
}

func extractCircuits(mapping map[Coord]*[]Coord) []*[]Coord {
	added := make(set)
	circuits := make([]*[]Coord, 0)
	for _, cPtr := range mapping {
		_, ok := added[cPtr]
		if !ok {
			circuits = append(circuits, cPtr)
			added[cPtr] = true
		}
	}
	return circuits
}

func CreateCircuits(dists []CoordPair, coords []Coord, connections int) ([]*[]Coord, CoordPair) {
	slices.SortFunc(dists, func(a, b CoordPair) int { return cmp.Compare(a.dist, b.dist) })
	circMap := make(map[Coord]*[]Coord, 0)
	var circuits []*[]Coord
	var pair CoordPair
	for _, pair = range dists[:connections] {
		c1c, okc1 := circMap[pair.c1]
		c2c, okc2 := circMap[pair.c2]
		if okc1 && okc2 && c1c != c2c {
			// Both present in a different circuit
			circMap = MergeCircuits(pair, circMap)
		} else if okc1 && !okc2 {
			// c1 present in a circuit
			*circMap[pair.c1] = append(*circMap[pair.c1], pair.c2)
			circMap[pair.c2] = circMap[pair.c1]
		} else if !okc1 && okc2 {
			// c2 present in a circuit
			*circMap[pair.c2] = append(*circMap[pair.c2], pair.c1)
			circMap[pair.c1] = circMap[pair.c2]
		} else if !okc1 && !okc2 {
			// None present in a circuit
			circuit := []Coord{pair.c1, pair.c2}
			circMap[pair.c1] = &circuit
			circMap[pair.c2] = &circuit
		}
		circuits = extractCircuits(circMap)
		if len(circuits) == 1 && len(circMap) == len(coords) {
			break
		}
	}
	return circuits, pair
}

func multiplyLargest(circuits []*[]Coord, n int) int {
	slices.SortFunc(circuits, func(a, b *[]Coord) int {
		return -cmp.Compare(len(*a), len(*b))
	})
	prod := 1
	for _, circ := range circuits[:n] {
		prod *= len(*circ)
	}
	return prod
}

func part1(input string) int {
	distances, coords := parseInput(input)
	circuits, _ := CreateCircuits(distances, coords, 1000)
	return multiplyLargest(circuits, 3)
}

func part2(input string) int {
	distances, coords := parseInput(input)
	_, lastLink := CreateCircuits(distances, coords, len(distances))
	return int(lastLink.c1.x) * int(lastLink.c2.x)
}

func main() {
	input := util.GetInput("day08.txt")

	fmt.Println("Part 1:")
	res := part1(input)
	fmt.Printf("\t%d\n", res)
	fmt.Println("Part 2:")
	res = part2(input)
	fmt.Printf("\t%d\n", res)
}
