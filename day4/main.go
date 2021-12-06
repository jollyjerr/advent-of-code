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

	for i := range data.draws {
		for j, board := range data.boards {
			if checkBoard(data.draws[:i], board) {
				fmt.Println(j)
				break
			}
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
	for i, val := range draws {
		if line[i] != val {
			return false
		}
	}
	return true
}

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
