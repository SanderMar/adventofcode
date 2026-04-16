package main

import (
	"reflect"
	"testing"
)

var example = `..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.`

type testCase struct {
	name  string
	input string
	want  int
}

func TestPart1(t *testing.T) {
	tests := []testCase{
		{
			name:  "exmaple",
			input: example,
			want:  13,
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
			input: example,
			want:  43,
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
		name    string
		input   string
		wantArr []int
		wantLen int
	}{
		{
			name:    "input1",
			input:   "..@.\n@@.@\n.@.@",
			wantArr: []int{0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1},
			wantLen: 4,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			gotArr, gotLen := parseInput(tt.input)
			if gotLen != tt.wantLen || !reflect.DeepEqual(tt.wantArr, gotArr) {
				t.Errorf("parseInput = %v, %v; want %v, %v", gotArr, gotLen, tt.wantArr, tt.wantLen)
			}
		})
	}
}

func TestGenNeighbors(t *testing.T) {
	tests := []struct {
		name   string
		inIdx  int
		inRLen int
		inGLen int
		want   []int
	}{
		{
			name:   "all",
			inIdx:  4,
			inRLen: 3,
			inGLen: 9,
			want:   []int{0, 1, 2, 3, 5, 6, 7, 8},
		},
		{
			name:   "LeftLower",
			inIdx:  0,
			inRLen: 3,
			inGLen: 9,
			want:   []int{1, 3, 4},
		},
		{
			name:   "RightUpper",
			inIdx:  8,
			inRLen: 3,
			inGLen: 9,
			want:   []int{4, 5, 7},
		},
		{
			name:   "Left",
			inIdx:  3,
			inRLen: 3,
			inGLen: 9,
			want:   []int{0, 1, 4, 6, 7},
		},
		{
			name:   "Left",
			inIdx:  5,
			inRLen: 3,
			inGLen: 9,
			want:   []int{1, 2, 4, 7, 8},
		},
		{
			name:   "Lower",
			inIdx:  1,
			inRLen: 3,
			inGLen: 9,
			want:   []int{0, 2, 3, 4, 5},
		},
		{
			name:   "Upper",
			inIdx:  7,
			inRLen: 3,
			inGLen: 9,
			want:   []int{3, 4, 5, 6, 8},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := genNeighbors(tt.inIdx, tt.inRLen, tt.inGLen); !reflect.DeepEqual(
				got,
				tt.want,
			) {
				t.Errorf("parseInput = %v; want %v", got, tt.want)
			}
		})
	}
}
