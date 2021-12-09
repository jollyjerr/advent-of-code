package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	data := loadData()
	partOne(data)
	partTwo(data)
}

type node struct {
	value   int
	edges   []string
	basinId string
}

func partTwo(data [][]int) {
	graph := make(map[string]node)

	for i, row := range data {
		for j, val := range row {
			graph[getNodeId(i, j)] = node{
				value:   val,
				edges:   getEdges(i, j, row, data),
				basinId: "none",
			}
		}
	}

	for key, node := range graph {
		exploreFrontier(node, key, graph)
	}

	fmt.Println(graph)
}

func getNodeId(i, j int) string {
	return fmt.Sprintf("%v,%v", i, j)
}

func exploreFrontier(node node, key string, graph map[string]node) {
	if node.value < 9 {
		for _, candidate := range node.edges {
			if graph[candidate].value < node.value {
				if graph[candidate].basinId != "none" {
					node.basinId = graph[candidate].basinId
					exploreFrontier(graph[candidate], graph[candidate].basinId, graph)
				} else {
					node.basinId = key
					copy := graph[candidate]
					copy.basinId = key
					graph[candidate] = copy
					exploreFrontier(graph[candidate], key, graph)
				}
			}
		}
	}
}

func getEdges(i, j int, row []int, data [][]int) []string {
	switch locate(i, j, len(row)-1, len(data)-1) {
	case "topLeft":
		return []string{getNodeId(i+1, j), getNodeId(i, j+1)}
	case "topRight":
		return []string{getNodeId(i+1, j), getNodeId(i, j-1)}
	case "top":
		return []string{getNodeId(i+1, j), getNodeId(i, j-1), getNodeId(i, j+1)}
	case "bottomLeft":
		return []string{getNodeId(i-1, j), getNodeId(i, j+1)}
	case "bottomRight":
		return []string{getNodeId(i-1, j), getNodeId(i, j-1)}
	case "bottom":
		return []string{getNodeId(i-1, j), getNodeId(i, j+1), getNodeId(i, j-1)}
	case "left":
		return []string{getNodeId(i+1, j), getNodeId(i, j+1), getNodeId(i-1, j)}
	case "right":
		return []string{getNodeId(i+1, j), getNodeId(i, j-1), getNodeId(i-1, j)}
	default:
		return []string{getNodeId(i+1, j), getNodeId(i, j+1), getNodeId(i-1, j), getNodeId(i, j-1)}
	}
}

func partOne(data [][]int) {
	lowPoints := make([]int, 0)
	for i, row := range data {
		for j, val := range row {
			switch locate(i, j, len(row)-1, len(data)-1) {
			case "topLeft":
				if lessThanRight(val, j, row) && lessThanBottom(val, i, j, data) {
					lowPoints = append(lowPoints, val)
				}
			case "topRight":
				if lessThanLeft(val, j, row) && lessThanBottom(val, i, j, data) {
					lowPoints = append(lowPoints, val)
				}
			case "top":
				if lessThanLeft(val, j, row) && lessThanBottom(val, i, j, data) && lessThanRight(val, j, row) {
					lowPoints = append(lowPoints, val)
				}
			case "bottomLeft":
				if lessThanTop(val, i, j, data) && lessThanRight(val, j, row) {
					lowPoints = append(lowPoints, val)
				}
			case "bottomRight":
				if lessThanLeft(val, j, row) && lessThanTop(val, i, j, data) {
					lowPoints = append(lowPoints, val)
				}
			case "bottom":
				if lessThanLeft(val, j, row) && lessThanTop(val, i, j, data) && lessThanRight(val, j, row) {
					lowPoints = append(lowPoints, val)
				}
			case "left":
				if lessThanTop(val, i, j, data) && lessThanRight(val, j, row) && lessThanBottom(val, i, j, data) {
					lowPoints = append(lowPoints, val)
				}
			case "right":
				if lessThanLeft(val, j, row) && lessThanTop(val, i, j, data) && lessThanBottom(val, i, j, data) {
					lowPoints = append(lowPoints, val)
				}
			case "middle":
				if lessThanLeft(val, j, row) && lessThanTop(val, i, j, data) && lessThanRight(val, j, row) && lessThanBottom(val, i, j, data) {
					lowPoints = append(lowPoints, val)
				}
			}
		}
	}
	fmt.Println(calculateRisk(lowPoints))
}

func calculateRisk(lowPoints []int) int {
	risk := 0
	for _, val := range lowPoints {
		risk += (val + 1)
	}
	return risk
}

func locate(i, j, rowLen, colLen int) string {
	location := "middle"

	if i == 0 {
		if j == 0 {
			location = "topLeft"
		} else if j == rowLen {
			location = "topRight"
		} else {
			location = "top"
		}
	} else if i == colLen {
		if j == 0 {
			location = "bottomLeft"
		} else if j == rowLen {
			location = "bottomRight"
		} else {
			location = "bottom"
		}
	} else if j == 0 {
		location = "left"
	} else if j == rowLen {
		location = "right"
	}

	return location
}

func lessThanLeft(val, j int, row []int) bool {
	return val < row[j-1]
}
func lessThanTop(val, i, j int, data [][]int) bool {
	return val < data[i-1][j]
}
func lessThanRight(val, j int, row []int) bool {
	return val < row[j+1]
}
func lessThanBottom(val, i, j int, data [][]int) bool {
	return val < data[i+1][j]
}

func loadData() [][]int {
	var data [][]int
	f, _ := os.Open("./day9/testData.txt")
	defer f.Close()
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		row := strings.Split(scanner.Text(), " ")
		realRow := make([]int, 0)
		for _, val := range row {
			for _, digit := range val {
				num, _ := strconv.Atoi(string(digit))
				realRow = append(realRow, num)
			}
			data = append(data, realRow)
		}
	}
	return data
}
