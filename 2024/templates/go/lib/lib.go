package lib

import (
	"strings"
)

func split_lines(input string) []string {
	var lines []string
	for _, line := range strings.Split(input, "\n") {
		if len(line) > 0 {
			lines = append(lines, line)
		}
	}
	return lines
}

func Part1(input string) string {
	return input
}
