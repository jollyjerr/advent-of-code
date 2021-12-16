package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {
	data := loadData()
	data.findLowestRiskPath(1, data.totalNodes())
}

type Graph [][]int

func (graph Graph) findLowestRiskPath(start, end int) int {
	riskLevels := make([]int, len(graph))
	visited := make([]bool, len(graph))
	for i := range riskLevels {
		riskLevels[i] = math.MaxInt
		visited[i] = false
	}

	// The starting position is never entered, so its risk is not counted
	riskLevels[start] = 0

	visitedCount := 0
	currentNode := start
	for visitedCount < graph.totalNodes() {
		frontier := graph.adjacent(currentNode)

	}
	return 1
}

func (graph Graph) totalNodes() int {
	return len(graph) * len(graph)
}

func (graph Graph) adjacent(index int) []int {
	candidates := make([]int, 0)
	x, y := graph.cords(index)
	if x > 0 {
		candidates = append(candidates, index-1)
	}
	if x < (len(graph) - 1) {
		candidates = append(candidates, index+1)
	}
	if y > 0 {
		candidates = append(candidates, index-(len(graph)))
	}
	if y < (len(graph) - 1) {
		candidates = append(candidates, index+(len(graph)))
	}
	return candidates
}

func (graph Graph) cords(index int) (int, int) {
	copy := index - 1
	x := 0
	y := 0
	if copy < len(graph)-1 {
		x = copy
	} else {
		for copy > len(graph)-1 {
			y++
			copy -= len(graph)
		}
		x = copy
	}
	return x, y
}

func (graph Graph) getRiskLevel(index int) int {
	x, y := graph.cords(index)
	fmt.Println("cords for", index, x, y)
	return graph[y][x]
}

func loadData() Graph {
	data := make(Graph, 0)
	if f, err := os.Open("./day15/testData.txt"); err == nil {
		defer f.Close()
		scanner := bufio.NewScanner(f)
		for scanner.Scan() {
			rowstring := strings.Split(scanner.Text(), "")
			row := make([]int, 0)
			for _, item := range rowstring {
				if num, err := strconv.Atoi(item); err == nil {
					row = append(row, num)
				}
			}
			data = append(data, row)
		}
	}
	return data
}
