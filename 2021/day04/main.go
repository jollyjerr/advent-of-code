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

func partOne(data Game) {
	win := 0
	winner := false
	var draws []int

	for i := 0; i < len(data.draws); i++ {
		for j, board := range data.boards {
			if checkBoard(data.draws[:i+1], board) {
				win = j
				winner = true
				draws = data.draws[:i+1]
				break
			}
		}
		if winner {
			break
		}
	}

	fmt.Println("Part one", score(data.boards[win], draws))
}

func partTwo(data Game) {
	var winners []int
	done := false

	for i := 0; i < len(data.draws); i++ {
		for j, board := range data.boards {
			if checkBoard(data.draws[:i+1], board) {
				if !inSlice(j, winners) {
					winners = append(winners, j)
					fmt.Println(winners)
				}
				if len(winners) == len(data.boards) {
					fmt.Println("Part Two", score(board, data.draws[:i+1]))
					done = true
					break
				}
			}
		}
		if done {
			break
		}
	}
}

func checkBoard(draws []int, board Board) bool {
	for i := 0; i < len(board); i++ {
		valid := checkLine(draws, board[i])
		if valid {
			return true
		}
		col := make([]int, 0)
		for _, row := range board {
			col = append(col, row[i])
		}
		valid = checkLine(draws, col)
		if valid {
			return true
		}
	}
	return false
}
func checkLine(draws []int, line []int) bool {
	if len(draws) < 5 {
		return false
	}
	for _, val := range line {
		if !inSlice(val, draws) {
			return false
		}
	}
	return true
}

func inSlice(element int, slice []int) bool {
	result := false
	for _, x := range slice {
		if x == element {
			result = true
		}
	}
	return result
}

func score(board Board, draws []int) int {
	sum := 0
	for i := 0; i < len(board); i++ {
		for j := 0; j < len(board[i]); j++ {
			if !inSlice(board[i][j], draws) {
				sum += board[i][j]
			}
		}
	}
	return sum * draws[len(draws)-1]
}

//

type Board [][]int
type Game struct {
	draws  []int
	boards []Board
}

func loadData() Game {
	var game Game
	f, _ := os.Open("./day4/data.txt")
	defer f.Close()
	scanner := bufio.NewScanner(f)
	line := 0
	boardNum := -1
	for scanner.Scan() {
		if line == 0 {
			draws := strings.Split(scanner.Text(), ",")
			for _, val := range draws {
				num, _ := strconv.Atoi(val)
				game.draws = append(game.draws, num)
			}
			line++
		} else {
			if scanner.Text() == "" {
				boardNum++
				game.boards = append(game.boards, make(Board, 0))
			} else {
				row := strings.Split(scanner.Text(), " ")
				realRow := make([]int, len(row))
				for i, val := range row {
					num, _ := strconv.Atoi(val)
					realRow[i] = num
				}
				game.boards[boardNum] = append(game.boards[boardNum], realRow)
			}
		}
	}
	return game
}
