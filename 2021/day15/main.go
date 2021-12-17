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

	riskLevels, _ := data.dijsktra(0, data.totalNodes()-1)
	fmt.Println("Part one", riskLevels[len(riskLevels)-1])

	data = data.expand(5)
	riskLevels2, _ := data.dijsktra(0, data.totalNodes()-1)
	// nextNode := paths[len(paths)-1]
	// for nextNode > 0 {
	// 	fmt.Println(nextNode, riskLevels2[nextNode])
	// 	backstep := paths[nextNode]
	// 	nextNode = backstep
	// }
	fmt.Println("Part two", riskLevels2[len(riskLevels2)-1]-3)
}

type Graph [][]int

func (graph Graph) dijsktra(start, end int) ([]int, []int) {
	riskLevels := make([]int, len(graph)*len(graph))
	prevVert := make([]int, len(riskLevels))
	visited := make([]bool, len(riskLevels))
	for i := range riskLevels {
		riskLevels[i] = math.MaxInt
		visited[i] = false
	}

	// The starting position is never entered, so its risk is not counted
	riskLevels[start] = 0
	prevVert[start] = start

	visitedCount := 0
	currentNode := start
	for visitedCount < graph.totalNodes() {
		visited[currentNode] = true
		visitedCount++

		frontier := graph.adjacent(currentNode)
		// leastRisky := []int{math.MaxInt, math.MaxInt} // not sure if greedy approach works?
		for _, val := range frontier {
			if !visited[val] {
				risk := graph.getRiskLevel(val) + riskLevels[currentNode]
				if risk < riskLevels[val] {
					riskLevels[val] = risk
					prevVert[val] = currentNode
				}
				// if risk < leastRisky[1] {
				// 	leastRisky = []int{val, risk}
				// }
			}
		}
		currentNode++
	}
	return riskLevels, prevVert
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
	x := 0
	y := 0
	if index < len(graph)-1 {
		x = index
	} else {
		for index > len(graph)-1 {
			y++
			index -= len(graph)
		}
		x = index
	}
	return x, y
}

func (graph Graph) getRiskLevel(index int) int {
	x, y := graph.cords(index)
	return graph[y][x]
}

func (graph Graph) expand(multiplier int) Graph {
	newGraph := make(Graph, 0)
	newGraph = append(newGraph, graph...)
	for i := 0; i < multiplier-1; i++ {
		slicePoint := 0
		if i > 0 {
			slicePoint = len(newGraph) - len(graph)
		}
		for _, row := range newGraph[slicePoint:] {
			newRow := incrementRow(row)
			newGraph = append(newGraph, newRow)
		}
	}
	newNewGraph := make(Graph, 0)
	newNewGraph = append(newNewGraph, newGraph...)
	for i := 0; i < multiplier-1; i++ {
		for j, row := range newNewGraph {
			slicePoint := 0
			if i > 0 {
				slicePoint = i * len(graph)
			}
			newRow := incrementRow(row[slicePoint:])
			newNewGraph[j] = append(newNewGraph[j], newRow...)
		}
	}
	return newNewGraph
}

func incrementRow(row []int) []int {
	newRow := make([]int, 0)
	for _, val := range row {
		if val == 9 {
			newRow = append(newRow, 1)
		} else {
			newRow = append(newRow, val+1)
		}
	}
	return newRow
}

func loadData() Graph {
	data := make(Graph, 0)
	if f, err := os.Open("./day15/data.txt"); err == nil {
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
