package lib

import (
	"os"
	"testing"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func TestPart1(t *testing.T) {
	data, err := os.ReadFile("./test_input.txt")
	check(err)

	result := Part1(string(data))
	expected := "41"
	if result != expected {
		t.Errorf("expected %q, found %q", expected, result)
	}

}
