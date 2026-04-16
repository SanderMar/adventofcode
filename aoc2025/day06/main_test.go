package main

import (
	"testing"
)

var example = `123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  `

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
			want:  4277556,
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
			want:  3263827,
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
