package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	data := loadData()
	lowPoints := partOne(data)
	partTwo(data, lowPoints)
}

type node struct {
	value    int
	edges    []string
	searched bool
}

func partTwo(data [][]int, lowPoints []string) {
	graph := make(map[string]node)

	for i, row := range data {
		for j, val := range row {
			graph[getNodeId(i, j)] = node{
				value:    val,
				edges:    getEdges(i, j, row, data),
				searched: false,
			}
		}
	}

	sizes := make([]int, 0)
	for _, key := range lowPoints {
		size := 0
		search(graph, key, &size)
		sizes = append(sizes, size)
	}

	fmt.Println("part two", calculateBasinRisk(sizes))
}

func calculateBasinRisk(basinSizes []int) int {
	sort.Ints(basinSizes)
	largest := basinSizes[len(basinSizes)-3:]
	fmt.Println(largest)
	total := largest[0]
	for i := 2; i < len(basinSizes); i++ {
		total *= basinSizes[i]
	}
	return total
}

func search(graph map[string]node, key string, counter *int) {
	*counter++
	markNode(graph, key)
	for _, candidate := range graph[key].edges {
		if graph[candidate].value < 9 && !graph[candidate].searched {
			search(graph, candidate, counter)
		}
	}
}

func markNode(graph map[string]node, key string) {
	copy := graph[key]
	copy.searched = true
	graph[key] = copy
}

func getNodeId(i, j int) string {
	return fmt.Sprintf("%v,%v", i, j)
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

func partOne(data [][]int) []string {
	lowPoints := make([]int, 0)
	lowPointCords := make([]string, 0)
	for i, row := range data {
		for j, val := range row {
			switch locate(i, j, len(row)-1, len(data)-1) {
			case "topLeft":
				if lessThanRight(val, j, row) && lessThanBottom(val, i, j, data) {
					lowPoints = append(lowPoints, val)
					lowPointCords = append(lowPointCords, getNodeId(i, j))
				}
			case "topRight":
				if lessThanLeft(val, j, row) && lessThanBottom(val, i, j, data) {
					lowPoints = append(lowPoints, val)
					lowPointCords = append(lowPointCords, getNodeId(i, j))
				}
			case "top":
				if lessThanLeft(val, j, row) && lessThanBottom(val, i, j, data) && lessThanRight(val, j, row) {
					lowPoints = append(lowPoints, val)
					lowPointCords = append(lowPointCords, getNodeId(i, j))
				}
			case "bottomLeft":
				if lessThanTop(val, i, j, data) && lessThanRight(val, j, row) {
					lowPoints = append(lowPoints, val)
					lowPointCords = append(lowPointCords, getNodeId(i, j))
				}
			case "bottomRight":
				if lessThanLeft(val, j, row) && lessThanTop(val, i, j, data) {
					lowPoints = append(lowPoints, val)
					lowPointCords = append(lowPointCords, getNodeId(i, j))
				}
			case "bottom":
				if lessThanLeft(val, j, row) && lessThanTop(val, i, j, data) && lessThanRight(val, j, row) {
					lowPoints = append(lowPoints, val)
					lowPointCords = append(lowPointCords, getNodeId(i, j))
				}
			case "left":
				if lessThanTop(val, i, j, data) && lessThanRight(val, j, row) && lessThanBottom(val, i, j, data) {
					lowPoints = append(lowPoints, val)
					lowPointCords = append(lowPointCords, getNodeId(i, j))
				}
			case "right":
				if lessThanLeft(val, j, row) && lessThanTop(val, i, j, data) && lessThanBottom(val, i, j, data) {
					lowPoints = append(lowPoints, val)
					lowPointCords = append(lowPointCords, getNodeId(i, j))
				}
			case "middle":
				if lessThanLeft(val, j, row) && lessThanTop(val, i, j, data) && lessThanRight(val, j, row) && lessThanBottom(val, i, j, data) {
					lowPoints = append(lowPoints, val)
					lowPointCords = append(lowPointCords, getNodeId(i, j))
				}
			}
		}
	}
	fmt.Println("Part one", calculateRisk(lowPoints))
	return lowPointCords
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
	f, _ := os.Open("./day9/data.txt")
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
