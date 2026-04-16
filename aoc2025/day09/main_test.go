package main

import (
	"testing"
)

var example = `7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3`

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
			want:  50,
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
			want:  1,
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
