package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strings"
)

func main() {
	symbols := map[string]string{
		"(": ")",
		"[": "]",
		"{": "}",
		"<": ">",
	}
	scoreMapError := map[string]int{
		")": 3,
		"]": 57,
		"}": 1197,
		">": 25137,
	}
	scoreMapAutocomplete := map[string]int{
		")": 1,
		"]": 2,
		"}": 3,
		">": 4,
	}
	data := loadData()
	incompleteLines := partOne(data, symbols, scoreMapError)
	partTwo(incompleteLines, symbols, scoreMapAutocomplete)
}

func partTwo(data [][]string, symbols map[string]string, scoreMap map[string]int) {
	lineScores := make([]int, 0)
	for _, line := range data {
		score := 0
		stack := make([]string, 0)
		for _, char := range line {
			if _, ok := symbols[char]; ok {
				stack = append(stack, char)
			} else {
				// pop
				stack = stack[:len(stack)-1]
			}
		}

		for i := len(stack) - 1; i >= 0; i-- {
			score = score * 5
			score += scoreMap[symbols[stack[i]]]
		}

		lineScores = append(lineScores, score)
	}
	sort.Ints(lineScores)
	fmt.Println("Part two", lineScores[len(lineScores)/2])
}

func partOne(data [][]string, symbols map[string]string, scoreMap map[string]int) [][]string {
	dataCopy := make([][]string, 0)

	score := 0
	for _, line := range data {
		stack := make([]string, 0)
		loser := ""
		for _, char := range line {
			if _, ok := symbols[char]; ok {
				stack = append(stack, char)
			} else {
				// pop
				n := len(stack) - 1
				shouldClose := stack[n]
				stack = stack[:n]

				if symbols[shouldClose] != char {
					loser = char
					break
				}
			}
		}
		if loser != "" {
			score += scoreMap[loser]
		} else {
			dataCopy = append(dataCopy, line)
		}
	}

	fmt.Println("Part one", score)
	return dataCopy
}

func loadData() [][]string {
	var data [][]string
	if f, err := os.Open("./day10/data.txt"); err == nil {
		defer f.Close()
		scanner := bufio.NewScanner(f)
		for scanner.Scan() {
			data = append(data, strings.Split(scanner.Text(), ""))
		}
	}
	return data
}
