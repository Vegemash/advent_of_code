package main

import (
	"fmt"
	"lib"
	"os"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	data, err := os.ReadFile("./input.txt")
	check(err)
	fmt.Println(lib.Part1(string(data)))
}
