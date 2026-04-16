package main

import (
	"reflect"
	"testing"
)

var example = `987654321111111
811111111111119
234234234234278
818181911112111`

type testCase struct {
	name  string
	input [][]int
	want  int
}

func TestPart1(t *testing.T) {
	tests := []testCase{
		{
			name:  "exmaple",
			input: parseInput(example),
			want:  357,
		},
		{
			name:  "own example",
			input: parseInput("764891\n224245\n331112"),
			want:  169,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := part1(tt.input); got != tt.want {
				t.Errorf("part1 = %v; want %v", got, tt.want)
			}
		})
	}
}

func TestPart2(t *testing.T) {
	tests := []testCase{
		{
			name:  "exmaple",
			input: parseInput(example),
			want:  3121910778619,
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

func TestParseInput(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  [][]int
	}{
		{
			name:  "example 1",
			input: "123\n456\n789\n000",
			want:  [][]int{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}, {0, 0, 0}},
		},
		{
			name:  "example 2",
			input: "1230097\n456",
			want:  [][]int{{1, 2, 3, 0, 0, 9, 7}, {4, 5, 6}},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := parseInput(tt.input); !reflect.DeepEqual(tt.want, got) {
				t.Errorf("parseInput = %v; want %v", got, tt.want)
			}
		})
	}
}

func TestFindVal(t *testing.T) {
	tests := []struct {
		name  string
		input []int
		total int
		want  int
	}{
		{
			name:  "example t2 1",
			input: []int{0, 1, 2, 3},
			total: 2,
			want:  23,
		},
		{
			name:  "example t2 2",
			input: []int{3, 1, 2, 3},
			total: 2,
			want:  33,
		},
		{
			name:  "exmaple t3",
			input: []int{0, 1, 2, 4, 2, 1, 9, 3},
			total: 3,
			want:  493,
		},
		{
			name:  "example t4",
			input: []int{7, 6, 4, 8, 9, 1},
			total: 4,
			want:  7891,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := findVoltage(tt.input, tt.total); got != tt.want {
				t.Errorf("findVoltage(%v) = %v; want %v", tt.total, got, tt.want)
			}
		})
	}
}
