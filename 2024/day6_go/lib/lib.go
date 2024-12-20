package lib

import (
	"fmt"
	"slices"
	"strings"
)

type coord struct {
	x int
	y int
}

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
	lines := split_lines(input)
	grid := make([][]rune, len(lines))
	var pos = coord{0, 0}

	for y, line := range lines {
		if len(line) == 0 {
			print("ZERO LENGTH LINE")
			break
		}
		print("\n", y)
		grid[y] = make([]rune, len(line))
		for x, char := range line {
			if slices.Contains([]rune("<>v^"), char) {
				pos = coord{x, y}
			}
			print(string(char))
			grid[y][x] = char
		}
		if len(grid[y]) == 0 {
			panic("things")
		}
	}
	println("grid ", len(grid), " ", len(grid[0]))

	for {
		var done bool
		pos, done = step(pos, &grid)
		if done {
			break
		}

	}
	printgrid(&grid)
	count := 0

	for _, line := range grid {
		for _, char := range line {
			if string(char) == "X" {
				count += 1
			}
		}
	}
	return fmt.Sprint(count)

}

func printgrid(grid *[][]rune) {
	for _, line := range *grid {
		print("\n")
		for _, char := range line {
			print(string(char))
		}
	}
}

func step(pos coord, grid *[][]rune) (coord, bool) {
	var dir coord
	switch (*grid)[pos.y][pos.x] {
	case '^':
		dir = coord{0, -1}
	case 'v':
		dir = coord{0, 1}
	case '<':
		dir = coord{-1, 0}
	case '>':
		dir = coord{1, 0}
	default:
		println("=====")
		println(pos.x, pos.y, string((*grid)[pos.y][pos.x]))
		println("=====")
		panic(fmt.Sprintf("%q not in '<>v^', at (%q, %q)", (*grid)[pos.y][pos.x], pos.x, pos.y))
	}

	new_pos := add(pos, dir)

	if !contains(grid, new_pos) {
		(*grid)[pos.y][pos.x] = 'X'
		return coord{}, true
	}
	next_rune := (*grid)[new_pos.y][new_pos.x]

	if next_rune == '.' || next_rune == 'X' {
		(*grid)[new_pos.y][new_pos.x] = (*grid)[pos.y][pos.x]
		(*grid)[pos.y][pos.x] = 'X'
		return new_pos, false
	}

	if next_rune == '#' {
		switch (*grid)[pos.y][pos.x] {
		case '^':
			(*grid)[pos.y][pos.x] = '>'
		case 'v':
			(*grid)[pos.y][pos.x] = '<'
		case '<':
			(*grid)[pos.y][pos.x] = '^'
		case '>':
			(*grid)[pos.y][pos.x] = 'v'
		}
		return pos, false

	}
	return new_pos, false

}

func contains(grid *[][]rune, pos coord) bool {
	return len(*grid) > pos.y && pos.y >= 0 && len((*grid)[0]) > pos.x && pos.x >= 0

}

func add(a coord, b coord) coord {
	return coord{a.x + b.x, a.y + b.y}
}
