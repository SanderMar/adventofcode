package main

import (
	"reflect"
	"testing"
)

var example = `3-5
10-14
16-20
12-18

1
5
8
11
17
32`

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
			want:  3,
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
			want:  14,
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

func TestParseRanges(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  [][2]int
	}{
		{
			name:  "ExampleRanges",
			input: "3-5\n10-14\n16-20\n12-18",
			want:  [][2]int{{3, 5}, {10, 20}},
		},
		{
			name:  "Enclosed",
			input: "1-10\n4-6",
			want:  [][2]int{{1, 10}},
		},
		{
			name:  "NoOverlap",
			input: "1-3\n4-6\n100-111",
			want:  [][2]int{{1, 3}, {4, 6}, {100, 111}},
		},
		{
			name:  "ReverseExtend",
			input: "10-20\n8-12",
			want:  [][2]int{{8, 20}},
		},
		{
			name:  "EqualBounds",
			input: "10-20\n20-30",
			want:  [][2]int{{10, 30}},
		},
	}
	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			if got := parseRanges(tc.input); !reflect.DeepEqual(tc.want, got) {
				t.Errorf("parseRanges = %v; want %v", got, tc.want)
			}
		})
	}
}
