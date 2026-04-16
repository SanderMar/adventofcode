package main

import (
	"math"
	"testing"
)

var example = `162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689`

type testCase struct {
	name  string
	input string
	want  int
}

func TestPart1(t *testing.T) {
	tests := []struct {
		name        string
		input       string
		connections int
		multiply    int
		want        int
	}{
		{
			name:        "exmaple",
			input:       example,
			connections: 10,
			multiply:    3,
			want:        40,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			dists, coords := parseInput(tt.input)
			cirquits, _ := CreateCircuits(dists, coords, tt.connections)
			if got := multiplyLargest(cirquits, tt.multiply); got != tt.want {
				t.Errorf("part1 = %v; want %v", got, tt.want)
			}
		})
	}
}

func TestPart2(t *testing.T) {
	tests := []testCase{
		{
			name:  "exmaple",
			input: example,
			want:  25272,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := part2(tt.input); got != tt.want {
				t.Errorf("part2 = %v; want %v", got, tt.want)
			}
		})
	}
}

func TestCreateCoord(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  Coord
	}{
		{
			name:  "HappyDay",
			input: "10,20,30",
			want:  Coord{10, 20, 30},
		},
		{
			name:  "NegCoord",
			input: "-10,20,30",
			want:  Coord{-10, 20, 30},
		},
		{
			name:  "LongCoords",
			input: "10000,23127892,6428710",
			want:  Coord{10000, 23127892, 6428710},
		},
	}
	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			if got := createCoord(tc.input); got != tc.want {
				t.Errorf("createCoord = %v; want %v", got, tc.want)
			}
		})
	}
}

func TestCoordDist(t *testing.T) {
	ref := Coord{0, 0, 0}
	tests := []struct {
		name  string
		other Coord
		want  float64
	}{
		{
			name:  "{1 0 0}",
			other: Coord{1, 0, 0},
			want:  1.0,
		},
		{
			name:  "{0 1 0}",
			other: Coord{0, 1, 0},
			want:  1.0,
		},
		{
			name:  "{0 0 1}",
			other: Coord{0, 0, 1},
			want:  1.0,
		},
		{
			name:  "{1 10 5}",
			other: Coord{1, 10, 5},
			want:  11.22497,
		},
	}
	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			if got := ref.dist(tc.other); math.Abs(got-tc.want) >= 1e-4 {
				t.Errorf("%v.dist = %v; want %v", ref, got, tc.want)
			}
		})
	}
}
