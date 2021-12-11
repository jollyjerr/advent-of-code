package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	data := loadData()
	partOne(data)
	partTwo(data)
}

type Score struct {
	most  string
	least string
}

func getMostCommon(data []string, stringindex int) Score {
	count0 := 0
	count1 := 0
	for _, val := range data {
		if string(val[stringindex]) == "1" {
			count1++
		} else {
			count0++
		}
	}
	if count1 >= count0 {
		return Score{
			most:  "1",
			least: "0",
		}
	} else {
		return Score{
			most:  "0",
			least: "1",
		}
	}
}

func partOne(data []string) {
	var gamma []string
	var epsilon []string
	for i := 0; i < len(data[0]); i++ {
		score := getMostCommon(data, i)
		gamma = append(gamma, score.most)
		epsilon = append(epsilon, score.least)
	}

	gamdec, _ := strconv.ParseInt(strings.Join(gamma, ""), 2, 64)
	epdec, _ := strconv.ParseInt(strings.Join(epsilon, ""), 2, 64)

	fmt.Println(gamdec * epdec)
}

func partTwo(data []string) {
	oxygen := make([]string, len(data))
	copy(oxygen, data)

	digitToCheck := 0
	for len(oxygen) > 1 {
		score := getMostCommon(oxygen, digitToCheck)

		n := 0
		for _, val := range oxygen {
			if (string(val[digitToCheck])) == score.most {
				oxygen[n] = val
				n++
			}
		}
		oxygen = oxygen[:n]
		digitToCheck++
	}

	co2scrubber := make([]string, len(data))
	copy(co2scrubber, data)

	digitToCheck = 0
	for len(co2scrubber) > 1 {
		score := getMostCommon(co2scrubber, digitToCheck)

		n := 0
		for _, val := range co2scrubber {
			if (string(val[digitToCheck])) == score.least {
				co2scrubber[n] = val
				n++
			}
		}
		co2scrubber = co2scrubber[:n]
		digitToCheck++
	}

	oxydec, _ := strconv.ParseInt(oxygen[0], 2, 64)
	co2dec, _ := strconv.ParseInt(co2scrubber[0], 2, 64)

	fmt.Println(oxydec * co2dec)
}

func loadData() []string {
	var data []string
	f, err := os.Open("./day3/data.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		data = append(data, scanner.Text())
	}
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	return data
}
